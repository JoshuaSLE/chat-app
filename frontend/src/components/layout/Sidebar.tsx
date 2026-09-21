import { useState } from "react";
import { useRooms } from "../../hooks/useRooms";
import UserBar from "./UserBar";

export default function Sidebar() {
  const [view, setView] = useState<"dm" | "room">("dm");

  const { dms, groups, loading } = useRooms();
  const items = view === "dm" ? dms : groups;

  return (
    <aside>
      <div>
        <button onClick={() => setView("dm")}>Direct Messages</button>
        <button onClick={() => setView("room")}>Rooms</button>
      </div>

      <ul>
        {loading && <li>Loading...</li>}
        {!loading && items.length === 0 && <li>Nothing here yet</li>}
        {items.map((r) => (
          <li key={r.room_id}>
            <button>{r.room_name}</button>
          </li>
        ))}
      </ul>

      <UserBar />
    </aside>
  );
}
