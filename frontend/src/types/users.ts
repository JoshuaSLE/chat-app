export type UserResponse = {
  readonly username: string;
};

export type RegisterUser = {
  readonly username: string;
  readonly password: string;
  readonly bio?: string;
  readonly avatar?: string;
};

export type LoginUser = {
  readonly username: string;
  readonly password: string;
};

export type LoginUserResponse = {
  readonly access_token: string;
};

export type UpdateUser = {
  readonly username?: string;
  readonly password?: string;
  readonly bio?: string;
  readonly avatar?: string;
};

export type UpdateUserResponse = {
  readonly username: string;
  readonly bio?: string;
  readonly avatar_key?: string;
};

export type MeUserResponse = {
  readonly username: string;
  readonly bio?: string;
  readonly created_at: string;
  readonly last_seen?: string;
  readonly online: boolean;
  readonly avatar_key?: string;
};

export type SearchUser = {
  readonly username: string;
};

export type UserSearchResult = {
  readonly username: string;
  readonly bio?: string;
};

export type SearchUserResponse = UserSearchResult[];

export type StatusUser = {
  readonly username: string;
};

export type StatusUserResponse = {
  readonly last_seen?: string;
  readonly online: boolean;
};
