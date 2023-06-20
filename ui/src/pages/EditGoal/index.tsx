import React from "react";

export const EditGoal: React.FC = () => {
  return (
    <div>
      <div>Edit Goal</div>
      <div>put the existing action points for the goal here</div>
      <EditGoalActionPoint />
    </div>
  );
};

export const EditGoalActionPoint: React.FC = () => {
  return <div>popup - edit action point</div>;
};
