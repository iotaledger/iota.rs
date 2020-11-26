export declare class Client {
  constructor(node: string | string[])
  subscriber(): TopicSubscriber
  getInfo(): Promise<string>
  getMessage(): MessageFinder
}

export declare class MessageFinder {
  index(index: string): Promise<string>
  data(messageId: string): Promise<string>
  raw(messageId: string): Promise<string>
  children(messageId: string): Promise<string>
  metadata(messageId: string): Promise<string>
}

export declare type Callback = (err: any, data: any) => void

export declare class TopicSubscriber {
  topic(topic: string): TopicSubscriber
  topics(topic: string[]): TopicSubscriber
  subscribe(cb: Callback): TopicSubscriber
  unsubscribe(cb: Callback): TopicSubscriber
}
