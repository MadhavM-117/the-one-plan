import React, { useState } from "react";
import { ContentContainer } from "src/components/Containers";
import { ActionPoint, Goal } from "src/types";
import styles from "./index.module.scss";

export const Home: React.FC = () => {
  return (
    <ContentContainer>
      <div>
        <h4>
          It is those who concentrate on but one thing at a time who advance in
          this world.
        </h4>
        <div>
          <button className={styles.createGoalButton}>Create New Goal</button>
        </div>
        <GoalDetailsContainer />
      </div>
    </ContentContainer>
  );
};

export const GoalDetailsContainer: React.FC = () => {
  const [goals, setGoals] = useState<Goal[]>([]);
  const [actionPoints, setActionPoints] = useState<ActionPoint[]>([]);

  return (
    <>
      <h4>Here's what you've been working on:</h4>
      {goals.map((g) => (
        <GoalPreview
          goal={g}
          actionPoints={actionPoints.filter((a) => a.goalId === g.id)}
        />
      ))}
    </>
  );
};

export interface GoalPreviewProps {
  goal: Goal;
  actionPoints: ActionPoint[];
}

export const GoalPreview: React.FC<GoalPreviewProps> = ({ goal }) => {
  return (
    <div>
      <div>Goal - {goal.title}</div>
      <div>{goal.description}</div>
    </div>
  );
};
