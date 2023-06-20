/**
 * @file Create a new goal.
 */
import React from "react";

export const CreateGoal: React.FC = () => {
  return (
    <div>
      <div>Create Goal</div>
      <div>
        <label>What do you want to call this?</label>
        <input />
      </div>
      <div>
        <label>Describe the goal in a bit more detail.</label>
        <textarea />
      </div>
      <div>
        <button>Save</button>
      </div>
    </div>
  );
};
