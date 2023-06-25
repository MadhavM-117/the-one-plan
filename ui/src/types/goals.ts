export interface Goal {
  id: string;
  userId: string;
  title: string;
  description: string;
  createdAt: string;
  updatedAt: string;
}

export interface ActionPoint {
  id: string;
  goalId: string;
  completed: boolean;
  actionPointType: string;
  description: string;
  createdAt: string;
  updatedAt: string;
}

export interface UpdateGoalRequest {
  title?: string;
  description?: string;
}

export interface CreateGoalRequest {
  title: string;
  description: string;
}

export interface CreateActionPointRequest {
  actionPointType: string;
  description: string;
}

export interface UpdateActionPointRequest {
  description?: string;
  completed?: boolean;
}
