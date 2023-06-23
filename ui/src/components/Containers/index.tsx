import React from "react";
import styles from "./index.module.scss";

export interface ContainerProps {
  children: React.ReactNode;
}
export const ContentContainer: React.FC<ContainerProps> = ({ children }) => {
  return <div className={styles.contentContainer}>{children}</div>;
};
