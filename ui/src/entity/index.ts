export interface Timeline {
    id?: number,
    type?: string,
    title?: string,
    content?: string,
    time?: string,
}
export interface Article {
    id?: number,
    user_id?: number,
    category_id?: number,
    article_tags?: string,
    title?: string,
    cover?: string,
    content?: string,
    markdown?: string,
    html?: string,
    publish_time?: string,
    update_time?: string,
    status?: number,
    views?: number,
    likes?: number,
}

export interface ArticleTags {
    id?: number,
    article_id?: number,
    tag_id?: number,
}

export interface Category {
    id?: number,
    category_name?: string,
    ioc?: string,
    create_time?: string,
    update_time?: string,
}

export interface Comment {
    id?: number,
    comment_id?: number,
    article_id?: number,
    user_id?: number,
    qq?: string,
    web_url?: string,
    content?: string,
    create_time?: string,
    update_time?: string,
}

export interface Permissions {
    id?: number,
    permission_name?: string,
    description?: string,
}

export interface RolePermissions {
    id?: number,
    roles_id?: number,
    permissions_id?: number,
}

export interface Roles {
    id?: number,
    role_name?: string,
    description?: string,
}

export interface Tags {
    id?: number,
    tag_name?: string,
    created_at?: string,
    updated_at?: string,
}

export interface User {
    id?: number,
    username?: string,
    password?: string,
    email?: string,
    nickname?: string,
    avatar?: string,
    create_time?: string,
    update_time?: string,
}

export interface UserRoles {
    id?: number,
    user_id?: number,
    role_id?: number,
}