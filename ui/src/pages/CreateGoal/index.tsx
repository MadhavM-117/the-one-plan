/**
 * @file Create a new goal.
 */
import React from "react";
import { ApiFactory } from "src/api";
import { ContentContainer } from "src/components/Containers";
import styles from "./index.module.scss";
const api = ApiFactory.getApi();
export const CreateGoal: React.FC = () => {
  return (
    <ContentContainer>
      <h3>Create Goal</h3>
      <div className={styles.inputBlock}>
        <label>What do you want to call this?</label>
        <input />
      </div>
      <div className={styles.inputBlock}>
        <label>Describe the goal in a bit more detail.</label>
        <textarea />
      </div>
      <div>
        <button>Save</button>
      </div>
    </ContentContainer>
  );
};
