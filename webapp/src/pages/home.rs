use crate::prelude::*;

/// App home page
pub(crate) fn page_home(_contexts: Contexts) -> Html {
    html! {
        <>
            {title_secondary!(format!("Welcome to {}", get_app_name()))}
            <SideImage image_pos={LeftOrRight::Left} src="https://cdn.myfi.ws/v/Vecteezy/website-development-web-design-programming-and-coding-php2.svg">
                <Paper>
                {paragraphs!(
                    format!("We are not currently working on {}, but expect to be working on it soon.", get_app_name()),
                    html!{
                        <>
                            {"In the meantime, visit our "}
                            <Link title="Stoic Dreams" href="https://www.stoicdreams.com">{"Stoic Dreams company website"}</Link>
                            {" to see our current projects."}
                        </>
                    },
                    html!{
                        <>
                            {"Also check out our "}
                            <Link title="Stoic Dreams" href="https://host.hatchmyweb.com">{"discounted domain registration and hosting services"}</Link>
                            {" powered by GoDaddy."}
                        </>
                    }
                )}
                </Paper>
            </SideImage>
        </>
    }
}
