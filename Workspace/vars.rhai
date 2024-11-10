// This file sets variables based on the results of the shared-init script.

const WORKSPACE_MEMBERS_VAR = "workspace_members";

const IS_WEB_VAR = "is_web";
const IS_DESKTOP_VAR = "is_desktop";
const IS_MOBILE_VAR = "is_mobile";

let workspace_members = [];

if variable::get(IS_WEB_VAR) {
    workspace_members += "web"
}

if variable::get(IS_DESKTOP_VAR) {
    workspace_members += "desktop"
}

if variable::get(IS_WEB_VAR) {
    workspace_members += "mobile"
}

variable::set(WORKSPACE_MEMBERS_VAR, workspace_members);