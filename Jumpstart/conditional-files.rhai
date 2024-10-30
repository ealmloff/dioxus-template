// This file handles conditional files for this specific sub-template.

let is_tailwind = variable::get("is_tailwind");
if !is_tailwind {
    file::delete("tailwind.config.js");
    file::delete("input.css");
}

let is_router = variable::get("is_router");
if !is_router {
    file::delete("src/pages");
    file::delete("src/components/navbar.rs");
    file::delete("assets/styling/navbar.css");
    file::delete("assets/styling/blog.css");
}

let is_fullstack = variable::get("is_fullstack");
if !is_fullstack {
    file::delete("src/components/calculator.rs");
}