import React from "react";
import { Outlet } from "react-router-dom";
import styles from "./index.module.scss";

export const Layout: React.FC = () => {
  return (
    <div className={styles.container}>
      <Header />
      <div className={styles.contentContainer}>
        <Outlet />
      </div>
    </div>
  );
};

export const Header: React.FC = () => {
  return <div className={styles.headerContainer}>The One Plan</div>;
};
