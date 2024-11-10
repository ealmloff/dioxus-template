// This file handles conditional files for this specific sub-template.

const IS_WEB_VAR = "is_web";
const IS_DESKTOP_VAR = "is_desktop";
const IS_MOBILE_VAR = "is_mobile";
const IS_FULLSTACK_VAR = "is_fullstack";

let is_web = variable::get(IS_WEB_VAR);
if !is_web {
    file::delete("web");
}

let is_desktop = variable::get(IS_DESKTOP_VAR);
if !is_desktop {
    file::delete("desktop");
}

let is_mobile = variable::get(IS_MOBILE_VAR);
if !is_mobile {
    file::delete("mobile");
}

let is_fullstack = variable::get(IS_FULLSTACK_VAR);
if !is_fullstack {
    file::delete("rpc");
}