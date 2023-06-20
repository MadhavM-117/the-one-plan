import React from "react";
import styles from "./index.module.scss";

export const Register: React.FC = () => {
  return (
    <div className={styles.container}>
      <div>
        <label>Name</label>
        <input />
      </div>
      <div>
        <label>Email</label>
        <input type="email" />
      </div>
      <div>
        <label>Password</label>
        <input type="password" />
      </div>
      <button>Register</button>
    </div>
  );
};
