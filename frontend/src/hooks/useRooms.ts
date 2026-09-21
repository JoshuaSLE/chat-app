import { useEffect, useState } from "react";
import { list } from "../api/room";
import type { ListRoomResponse } from "../types/rooms";

export function useRooms() {
  const [rooms, setRooms] = useState<ListRoomResponse[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    list()
      .then(({ data }) => setRooms(data))
      .finally(() => setLoading(false));
  }, []);

  return {
    dms: rooms.filter((r) => r.is_direct),
    groups: rooms.filter((r) => !r.is_direct),
    loading,
  };
}
