declare module "@xterm/xterm" {
  export interface IDisposable {
    dispose(): void;
  }

  export interface ITerminalOptions {
    cursorBlink?: boolean;
    fontSize?: number;
    theme?: {
      background?: string;
      foreground?: string;
    };
  }

  export class Terminal {
    constructor(options?: ITerminalOptions);
    cols: number;
    rows: number;
    loadAddon(addon: unknown): void;
    open(container: HTMLElement): void;
    write(data: string): void;
    onData(callback: (data: string) => void): IDisposable;
    dispose(): void;
  }
}

declare module "@xterm/addon-fit" {
  export class FitAddon {
    activate(terminal: unknown): void;
    dispose(): void;
    fit(): void;
  }
}
