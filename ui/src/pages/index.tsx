import { createBrowserRouter, Outlet } from "react-router-dom";

export const router = createBrowserRouter([
  {
    path: "/",
    element: (
      <div>
        Hello World! <Outlet />
      </div>
    ),
    children: [
      {
        index: true,
        element: <div>How do you do?</div>,
      },
    ],
  },
]);
