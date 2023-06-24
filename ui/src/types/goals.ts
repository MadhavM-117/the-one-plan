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
