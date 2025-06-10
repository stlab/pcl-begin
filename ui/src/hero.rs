use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const D3_GRAPH_JS: Asset = asset!("/assets/d3-graph.js");

#[component]
pub fn Hero() -> Element {
    let mut zoom_percentage = use_signal(|| 100i32);

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        document::Script {
            src: "https://jspm.dev/@spectrum-web-components/bundle/elements.js",
            r#type: "module",
            r#async: true,
        }

        // Add D3.js library (load synchronously)
        document::Script { src: "https://d3js.org/d3.v7.min.js" }

        sp-theme { scale: "medium", color: "light",
            // Main container with vertical layout
            div { style: "display: flex; flex-direction: column; gap: 24px; max-width: 1200px; margin: 0 auto; padding: 24px;",

                // Existing Image Size Dialog
                div {
                    id: "image-size-dialog",
                    style: "width: 800px; padding: 24px; border: 1px solid var(--spectrum-gray-300); border-radius: 12px; background: white; display: flex; flex-direction: column; gap: 20px;",

                    // Dialog Header
                    sp-heading { size: "L", style: "color: var(--spectrum-gray-800);", "Image size" }

                    // Main Content - Two Column Layout
                    div { style: "display: flex; gap: 24px;",
                        // Left Column - Image Preview
                        div { style: "flex: 1;",
                            // Image Preview
                            div { style: "width: 100%; aspect-ratio: 4/3; border: 1px solid var(--spectrum-gray-300); border-radius: 8px; background: var(--spectrum-gray-100); display: flex; align-items: center; justify-content: center; margin-bottom: 16px;",
                                sp-icon { name: "Image", size: "XXL" }
                            }

                            // Zoom Controls
                            div { style: "display: flex; align-items: center; justify-content: center; gap: 12px; margin-bottom: 16px;",
                                sp-button {
                                    variant: "secondary",
                                    size: "s",
                                    onclick: {
                                        let mut zoom = zoom_percentage.clone();
                                        move |_| {
                                            let current = zoom();
                                            if current > 10 {
                                                zoom.set(current - 10);
                                            }
                                        }
                                    },
                                    "-"
                                }
                                sp-body {
                                    size: "M",
                                    style: "color: var(--spectrum-gray-800); width: 60px; text-align: center;",
                                    "{zoom_percentage}%"
                                }
                                sp-button {
                                    variant: "secondary",
                                    size: "s",
                                    onclick: {
                                        let mut zoom = zoom_percentage.clone();
                                        move |_| {
                                            let current = zoom();
                                            if current < 500 {
                                                zoom.set(current + 10);
                                            }
                                        }
                                    },
                                    "+"
                                }
                            }

                            // Image Info
                            div { style: "text-align: center;",
                                div { style: "display: flex; justify-content: space-between; margin-bottom: 8px;",
                                    sp-body {
                                        size: "S",
                                        style: "color: var(--spectrum-gray-700);",
                                        "W × H"
                                    }
                                    sp-body { size: "S", "1343 px × 1366 px" }
                                }
                                div { style: "display: flex; justify-content: space-between;",
                                    sp-body {
                                        size: "S",
                                        style: "color: var(--spectrum-gray-700);",
                                        "Image size"
                                    }
                                    sp-body { size: "S", "5.25MB" }
                                }
                            }
                        }

                        // Right Column - Controls
                        div { style: "flex: 1; display: flex; flex-direction: column; gap: 20px;",
                            // Resample Toggle
                            div { style: "display: flex; align-items: center; gap: 12px;",
                                sp-switch { checked: "true" }
                                sp-field-label { size: "M", "Resample" }
                            }

                            // Mode Section
                            div {
                                sp-field-label { size: "S", "Mode" }
                                sp-picker {
                                    style: "width: 100%; margin-top: 4px;",
                                    value: 0,
                                    sp-menu-item { value: 0, "Automatic" }
                                    sp-menu-item { value: 1, "Bicubic" }
                                    sp-menu-item { value: 2, "Bilinear" }
                                    sp-menu-item { value: 3, "Nearest Neighbor" }
                                }
                            }

                            // Fit to Section
                            div {
                                sp-field-label { size: "S", "Fit to" }
                                sp-picker {
                                    style: "width: 100%; margin-top: 4px;",
                                    value: 0,
                                    sp-menu-item { value: 0, "Original" }
                                    sp-menu-item { value: 1, "Custom" }
                                    sp-menu-item { value: 2, "Screen" }
                                }
                            }

                            sp-divider { size: "S" }

                            // Image Size Section
                            div {
                                sp-field-label { size: "M", "Image size" }

                                // Units
                                div { style: "margin: 12px 0;",
                                    sp-field-label { size: "S", "Units" }
                                    sp-picker {
                                        style: "width: 100%; margin-top: 4px;",
                                        value: 0,
                                        sp-menu-item { value: 0, "Inches" }
                                        sp-menu-item { value: 1, "Centimeters" }
                                        sp-menu-item { value: 2, "Millimeters" }
                                        sp-menu-item { value: 3, "Pixels" }
                                    }
                                }

                                // Width and Height
                                div { style: "display: flex; gap: 12px; align-items: end; margin-bottom: 12px;",
                                    div { style: "flex: 1;",
                                        sp-field-label { size: "S", "Width" }
                                        sp-textfield {
                                            value: "18.65",
                                            style: "width: 100%;",
                                        }
                                    }
                                    div { style: "display: flex; align-items: center; height: 32px;",
                                        sp-icon { name: "Link", size: "S" }
                                    }
                                    div { style: "flex: 1;",
                                        sp-field-label { size: "S", "Height" }
                                        sp-textfield {
                                            value: "18.97",
                                            style: "width: 100%;",
                                        }
                                    }
                                }

                                // Resolution
                                div { style: "display: flex; gap: 12px; align-items: end;",
                                    div { style: "flex: 1;",
                                        sp-field-label { size: "S", "Resolution" }
                                        sp-textfield {
                                            value: "72",
                                            style: "width: 100%;",
                                        }
                                    }
                                    sp-picker { style: "width: 80px;", value: 0,
                                        sp-menu-item { value: 0, "ppi" }
                                        sp-menu-item { value: 1, "ppc" }
                                    }
                                }
                            }
                        }
                    }

                    // Action Buttons
                    div { style: "display: flex; gap: 12px; justify-content: flex-end; margin-top: 50px;",
                        sp-button { variant: "secondary", "I can't do this" }
                        sp-button { variant: "cta", "Here is my button" }
                    }
                }

                // New D3.js Force Graph Panel
                div {
                    id: "force-graph-panel",
                    style: "width: 800px; padding: 24px; border: 1px solid var(--spectrum-gray-300); border-radius: 12px; background: white; display: flex; flex-direction: column; gap: 16px;",

                    // Panel Header
                    sp-heading { size: "L", style: "color: var(--spectrum-gray-800);", "Network Graph" }

                    // Graph Container
                    div {
                        id: "d3-graph-container",
                        style: "width: 100%; height: 400px; border: 1px solid var(--spectrum-gray-300); border-radius: 8px; background: var(--spectrum-gray-50);",
                    }
                }
            }
        }

        // D3.js Force Graph Script (load after D3.js)
        document::Script { src: D3_GRAPH_JS }
    }
}
