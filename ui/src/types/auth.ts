export interface UserData {
  id: string;
  name: string;
  email: string;
  phoneNumber?: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface SignupRequest {
  name: string;
  emailId: string;
  password: string;
}
