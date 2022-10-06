/** The status of a Ledger Nano */
export interface LedgerNanoStatus {
    connected: boolean;
    locked: boolean;
    blindSigningEnabled: boolean;
    app?: LedgerApp;
    device?: LedgerDeviceType;
    bufferSize?: number;
}

/** The current opened app */
export interface LedgerApp {
    name: string;
    version: string;
}

/** The Ledger Device Type */
export enum LedgerDeviceType {
    LedgerNanoS = 'ledgerNanoS',
    LedgerNanoX = 'ledgerNanoX',
    LedgerNanoSPlus = 'ledgerNanoSPlus',
}
