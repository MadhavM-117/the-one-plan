import {
  ActionPoint,
  CreateActionPointRequest,
  CreateGoalRequest,
  Goal,
  UpdateActionPointRequest,
  UpdateGoalRequest
} from "src/types";
import wretch from "wretch";

export interface GoalApiInterface {
  getGoals: () => Promise<Goal[]>;
  createGoal: (request: CreateGoalRequest) => Promise<Goal>;
  editGoal: (goalId: string, request: UpdateGoalRequest) => Promise<Goal>;
  deleteGoal: (goalId: string) => Promise<void>;
  getActionPoints: (goalId: string) => Promise<ActionPoint[]>;
  createActionPoint: (goalId: string, request: CreateActionPointRequest) => Promise<ActionPoint>;
  editActionPoint: (goalId: string, request: UpdateActionPointRequest) => Promise<ActionPoint>;
  deleteActionPoint: (goalId: string, actionPointId: string) => Promise<void>;
}

export const GoalApiFactory = (baseUrl: string): GoalApiInterface => {
  return {
    getGoals: async () => {
      return await wretch(`${baseUrl}/goals`)
        .options({ credentials: "include" })
        .get()
        .json<Goal[]>();
    },
    createGoal: async (request: CreateGoalRequest) => {
      return await wretch(`${baseUrl}/goals`)
        .options({ credentials: "include" })
        .post(request)
        .json<Goal>();
    },
    editGoal: async (goalId: string, request: UpdateGoalRequest) => {
      return await wretch(`${baseUrl}/goals/${goalId}`)
        .options({ credentials: "include" })
        .patch(request)
        .json<Goal>();
    },
    deleteGoal: async (goalId: string) => {
      await wretch(`${baseUrl}/goals/${goalId}`)
        .options({ credentials: "include" })
        .delete();
    },
    getActionPoints: async (goalId: string) => {
      return await wretch(`${baseUrl}/goals/${goalId}/action_points`)
        .options({ credentials: "include" })
        .get()
        .json<ActionPoint[]>();
    },
    createActionPoint: async (goalId: string, request: CreateActionPointRequest) => {
      return await wretch(`${baseUrl}/goals/${goalId}/action_points`)
        .options({ credentials: "include" })
        .post(request)
        .json<ActionPoint>();
    },
    editActionPoint: async (goalId: string, request: UpdateActionPointRequest) => {
      return await wretch(`${baseUrl}/goals/${goalId}/action_points`)
        .options({ credentials: "include" })
        .patch(request)
        .json<ActionPoint>();
    },
    deleteActionPoint: async (goalId: string, actionPointId: string) => {
      await wretch(`${baseUrl}/goals/${goalId}/action_points/${actionPointId}`)
        .options({ credentials: "include" })
        .delete();
    },
  };
}
