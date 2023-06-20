import { createBrowserRouter, Outlet } from "react-router-dom";
import { CreateGoal } from "src/pages/CreateGoal";
import { EditGoal } from "src/pages/EditGoal";
import { Home } from "src/pages/Home";
import { Layout } from "src/pages/Layout";
import { Login } from "src/pages/Login";
import { Register } from "src/pages/Register";

export const router = createBrowserRouter([
  {
    path: "/login",
    element: <Login />,
  },
  {
    path: "/register",
    element: <Register />,
  },
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: "create-goal",
        element: <CreateGoal />,
      },
      {
        path: "goal/:goalId",
        element: <EditGoal />,
      },
    ],
  },
]);
