export type CreateMessage = {
  readonly body?: string;
  readonly attachment_key?: string;
  readonly attachment_type?: string;
};

export type CreateMessageResponse = {
  readonly id: string;
  readonly username: string;
  readonly body?: string;
  readonly attachment_key?: string;
  readonly attachment_type?: string;
  readonly created_at: string;
};

export type GetMessage = {
  readonly limit?: number;
  readonly before?: string;
};

export type Message = {
  readonly id: string;
  readonly username: string;
  readonly body?: string;
  readonly attachment_key?: string;
  readonly attachment_type?: string;
  readonly created_at: string;
};

export type GetMessageResponse = Message[];
