import { useEffect, useState } from "react";
import { useParams } from "react-router";
import * as roomApi from "../api/room";
import * as messageApi from "../api/message";
import type { MeRoomResponse } from "../types/rooms";
import type { GetMessageResponse } from "../types/messages";
import { formatTime } from "../utils/date";

export default function RoomPage() {
  const { roomId } = useParams<{ roomId: string }>();
  const [room, setRoom] = useState<MeRoomResponse | null>(null);
  const [messages, setMessages] = useState<GetMessageResponse>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [text, setText] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    if (!roomId) return;

    let ignore = false;

    Promise.all([roomApi.me(roomId), messageApi.get(roomId, {})])
      .then(([roomRes, msgRes]) => {
        if (ignore) return;
        setRoom(roomRes.data);
        setMessages(msgRes.data);
        setError(null);
        setLoading(false);
      })
      .catch(() => {
        if (ignore) return;
        setError("Failed to load room.");
        setLoading(false);
      });

    return () => {
      ignore = true;
    };
  }, [roomId]);

  const handleSubmit = async (e: React.SubmitEvent) => {
    e.preventDefault();
    const trimmed = text.trim();
    if (!trimmed || !roomId || isSubmitting) return;

    setIsSubmitting(true);
    try {
      const { data: newMessage } = await messageApi.create(roomId, {
        body: trimmed,
      });

      setMessages((prev) => [...prev, newMessage]);
      setText("");
    } catch {
      setError("Failed to send message.");
    } finally {
      setIsSubmitting(false);
    }
  };

  if (loading) return <p>Loading...</p>;
  if (error) return <p>{error}</p>;

  return (
    <div>
      <header>
        {room?.is_direct ? "Direct Message" : (room?.name ?? "Room")}
      </header>

      <ul>
        {messages.length === 0 && <li>No messages yet.</li>}
        {messages.map((m) => (
          <li key={m.id}>
            <p>{m.body}</p>
            <div>
              <span>{m.username}</span>
              <span>{formatTime(m.created_at)}</span>
            </div>
          </li>
        ))}
      </ul>

      <form onSubmit={handleSubmit}>
        <input
          type="text"
          name="message"
          value={text}
          onChange={(e) => setText(e.target.value)}
          placeholder="Message..."
          disabled={isSubmitting}
        />
        <button type="submit" disabled={isSubmitting || !text.trim()}>
          Send
        </button>
      </form>
    </div>
  );
}
