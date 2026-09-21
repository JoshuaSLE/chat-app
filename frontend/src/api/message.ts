import api from "./client";

import type {
  CreateMessage,
  CreateMessageResponse,
  GetMessage,
  GetMessageResponse,
} from "../types/messages";

export const create = (room_id: string, data: CreateMessage) =>
  api.post<CreateMessageResponse>(`/rooms/${room_id}/messages`, data);

export const get = (room_id: string, params: GetMessage) =>
  api.get<GetMessageResponse[]>(`/rooms/${room_id}/messages`, { params });
