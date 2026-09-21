import { useAuth } from "../../hooks/useAuth";

export default function UserBar() {
  const { user, logout } = useAuth();

  return (
    <div>
      <span>{user?.username}</span>
      <button onClick={logout}>Logout</button>
    </div>
  );
}
