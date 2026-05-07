use std::io::Write;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, bail, Context, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};

use super::model::{
    TerminalDrainResponse, TerminalLaunchInput, TerminalSessionSnapshot, TerminalSessionStatus,
};
use super::{build_launch_spec, io::spawn_output_pump};

struct ActiveTerminalSession {
    snapshot: TerminalSessionSnapshot,
    child: Box<dyn Child + Send + Sync>,
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    output: Arc<Mutex<String>>,
}

#[derive(Default)]
pub struct TerminalState {
    inner: Mutex<Option<ActiveTerminalSession>>,
}

impl TerminalState {
    pub fn start(&self, input: TerminalLaunchInput) -> Result<TerminalSessionSnapshot> {
        let spec = build_launch_spec(input)?;
        let _ = self.stop();

        let mut last_error = None;
        for program in &spec.program_candidates {
            let pty_system = native_pty_system();
            let pair = pty_system
                .openpty(PtySize {
                    rows: spec.rows,
                    cols: spec.cols,
                    pixel_width: 0,
                    pixel_height: 0,
                })
                .context("Failed to create PTY")?;
            let mut builder = CommandBuilder::new(program);
            builder.cwd(spec.working_directory.clone());

            match pair.slave.spawn_command(builder) {
                Ok(child) => {
                    let reader = pair
                        .master
                        .try_clone_reader()
                        .context("Failed to clone PTY reader")?;
                    let writer = pair
                        .master
                        .take_writer()
                        .context("Failed to take PTY writer")?;
                    let output = Arc::new(Mutex::new(String::new()));
                    let _output_pump = spawn_output_pump(reader, output.clone());

                    let snapshot = TerminalSessionSnapshot {
                        cli_key: spec.cli_key,
                        working_directory: spec
                            .working_directory
                            .to_string_lossy()
                            .replace('\\', "/"),
                        cols: spec.cols,
                        rows: spec.rows,
                        status: TerminalSessionStatus::Running,
                        message: None,
                    };

                    *self.inner.lock().unwrap() = Some(ActiveTerminalSession {
                        snapshot: snapshot.clone(),
                        child,
                        master: pair.master,
                        writer,
                        output,
                    });

                    return Ok(snapshot);
                }
                Err(error) => {
                    last_error = Some(error.to_string());
                }
            }
        }

        Err(anyhow!(
            "No supported executable was launchable: {}",
            last_error.unwrap_or_else(|| "unknown launch failure".to_string())
        ))
    }

    pub fn snapshot(&self) -> Result<Option<TerminalSessionSnapshot>> {
        let mut guard = self.inner.lock().unwrap();
        let Some(session) = guard.as_mut() else {
            return Ok(None);
        };
        refresh_session_status(session)?;
        Ok(Some(session.snapshot.clone()))
    }

    pub fn drain(&self) -> Result<TerminalDrainResponse> {
        let mut guard = self.inner.lock().unwrap();
        let Some(session) = guard.as_mut() else {
            return Ok(TerminalDrainResponse {
                output: String::new(),
                session: None,
            });
        };
        refresh_session_status(session)?;
        let mut output = session.output.lock().unwrap();
        let drained = std::mem::take(&mut *output);

        Ok(TerminalDrainResponse {
            output: drained,
            session: Some(session.snapshot.clone()),
        })
    }

    pub fn write_input(&self, input: &str) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        let session = guard
            .as_mut()
            .ok_or_else(|| anyhow!("No active terminal session"))?;
        refresh_session_status(session)?;
        if session.snapshot.status != TerminalSessionStatus::Running {
            bail!("Terminal session is not running");
        }
        session.writer.write_all(input.as_bytes())?;
        session.writer.flush()?;
        Ok(())
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<Option<TerminalSessionSnapshot>> {
        let mut guard = self.inner.lock().unwrap();
        let Some(session) = guard.as_mut() else {
            return Ok(None);
        };
        session.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        session.snapshot.cols = cols;
        session.snapshot.rows = rows;
        Ok(Some(session.snapshot.clone()))
    }

    pub fn stop(&self) -> Result<Option<TerminalSessionSnapshot>> {
        let mut guard = self.inner.lock().unwrap();
        let Some(mut session) = guard.take() else {
            return Ok(None);
        };
        let _ = session.child.kill();
        session.snapshot.status = TerminalSessionStatus::Stopped;
        session.snapshot.message = Some("Terminated by user".into());
        Ok(Some(session.snapshot))
    }
}

fn refresh_session_status(session: &mut ActiveTerminalSession) -> Result<()> {
    if session.snapshot.status != TerminalSessionStatus::Running {
        return Ok(());
    }

    if let Some(status) = session.child.try_wait()? {
        session.snapshot.status = TerminalSessionStatus::Stopped;
        session.snapshot.message = Some(format!("Process exited with {}", status));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drain_without_session_is_empty() {
        let state = TerminalState::default();
        let drained = state.drain().unwrap();

        assert!(drained.output.is_empty());
        assert!(drained.session.is_none());
    }

    #[test]
    fn write_input_without_session_reports_missing_session() {
        let state = TerminalState::default();
        let error = state.write_input("help\r").unwrap_err();

        assert!(error.to_string().contains("active terminal"));
    }
}
