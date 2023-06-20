import React from "react";
import styles from "./index.module.scss";
export const Login: React.FC = () => {
  return (
    <div className={styles.container}>
      <div>
        <label>Email</label>
        <input type="email" />
      </div>
      <div>
        <label>Password</label>
        <input type="password" />
      </div>
      <button>Login</button>
    </div>
  );
};
