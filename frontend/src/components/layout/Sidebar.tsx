import { useState } from "react";
import { useRooms } from "../../hooks/useRooms";
import { NavLink } from "react-router";
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
            <NavLink
              to={`/rooms/${r.room_id}`}
              className={({ isActive }) =>
                `block rounded px-3 py-2 text-sm ${
                  isActive
                    ? "bg-gray-700 text-white"
                    : "text-gray-300 hover:bg-gray-700"
                }`
              }
            >
              {r.room_name}
            </NavLink>
          </li>
        ))}
      </ul>

      <UserBar />
    </aside>
  );
}
