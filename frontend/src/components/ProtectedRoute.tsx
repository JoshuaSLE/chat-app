import { Navigate, Outlet, useLocation } from "react-router";
import { useAuth } from "../hooks/useAuth";

export function ProtectedRoute() {
  const { user, isLoading } = useAuth();
  const location = useLocation();

  if (isLoading) return <div>Loading...</div>;
  if (!user) return <Navigate to="/login" state={{ from: location }} replace />;

  return <Outlet />;
}
