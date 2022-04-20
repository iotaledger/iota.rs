export interface OutputResponse {
    messageId: string;
    transactionId: string;
    outputIndex: number;
    isSpent: boolean;
    milestoneIndexBooked: number;
    milestoneTimestampBooked: number;
    ledgerIndex: number;
    // TODO: Replace with proper type
    output: any;
}
