// src/actions/add.rs
use crate::utils::{find_project_root_by_marker, run_command};
use crate::vcpkg::{self, VCPKG_JSON_FILENAME};
use anyhow::{Context, Result, bail};

pub fn add_dependencies(
    dependencies_to_add: &[String],
    vcpkg_root_override: Option<String>,
) -> Result<()> {
    if dependencies_to_add.is_empty() {
        bail!("No dependencies specified to add.");
    }

    println!("Attempting to add dependencies: {:?}", dependencies_to_add);

    // 1. Find project root (where vcpkg.json should be)
    let project_root = find_project_root_by_marker(VCPKG_JSON_FILENAME)
        .context("Failed to find project root. Are you in a rig-managed project (look for vcpkg.json)?")?;
    
    println!("Operating in project root: {}", project_root.display());

    // 2. Locate vcpkg executable
    let vcpkg_paths = vcpkg::locate_and_verify_vcpkg(vcpkg_root_override)
        .context("Failed to locate vcpkg.")?;

    // 3. Ensure vcpkg.json exists. vcpkg add port will create it if it doesn't.
    //    For a 'rig add' command, it's more robust to ensure we are in a context
    //    where vcpkg.json is expected.
    let vcpkg_json_path = project_root.join(VCPKG_JSON_FILENAME);
    if !vcpkg_json_path.exists() {
        // While `vcpkg add port` can create vcpkg.json,
        // for `rig add`, it's better to be explicit that this command
        // is intended for existing vcpkg manifest projects.
        // Or, we could choose to initialize it like `vcpkg new --application`
        // but that might be unexpected for a simple `add`.
        // Let's require `vcpkg.json` to exist for now.
        bail!(
            "{} not found in project root ({}). Initialize vcpkg manifest first (e.g., via `rig new` or `vcpkg new --application`).",
            VCPKG_JSON_FILENAME,
            project_root.display()
        );
    }

    // 4. Construct and run the vcpkg command
    let mut args: Vec<&str> = vec!["add", "port"];
    let dep_refs: Vec<&str> = dependencies_to_add.iter().map(AsRef::as_ref).collect();
    args.extend_from_slice(&dep_refs);

    run_command(&vcpkg_paths.exe, &args, Some(&project_root))
        .with_context(|| format!("Failed to add vcpkg dependencies: {:?}", dependencies_to_add))?;

    println!(
        "Successfully added dependencies: {:?}. Update your CMakeLists.txt if necessary.",
        dependencies_to_add
    );
    println!("You might need to add 'find_package(<dependency> CONFIG REQUIRED)' and link against '<dependency>::<dependency>'.");

    Ok(())
}