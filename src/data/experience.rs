#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Experience {
    pub begin: String,
    pub end: String,
    pub title: String,
    pub company: String,
    pub company_link: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Experience {

    pub fn data() -> Vec<Experience> {
        vec![
            Experience {
                begin: String::from("05/2023"),
                end: String::from("10/2023"),
                title: String::from("Cloud Engineer Intern"),
                company: String::from("Battelle"),
                company_link: String::from("https://www.battelle.org/"),
                description: String::from(
                "I worked in Battelle's Cloud Engineering team to develop infrastructure to support cloud work \
                enviroments in Azure that uphold CMMC level 2 security requirements. I wrote Terraform code to deploy \
                a Microsoft Teams Remote App to replace the companies Webex for Government contract. I also \
                worked on their internal tools written in Bash to automate deployments by writing a terraform module for \
                pip and apt proxies deployed with Docker within environments and developed a CI/CD pipeline \
                for custom SKU images for remote desktops utilizing Chocolately."
                ),
                tags: vec![
                    String::from("Terraform"),
                    String::from("Bash"),
                    String::from("Azure"),
                    String::from("GitHub"),
                    String::from("Docker"),
                    String::from("CI/CD"),
                ],
            },
            Experience {
                begin: String::from("05/2022"),
                end: String::from("07/2022"),
                title: String::from("Software Engineer Intern"),
                company: String::from("Georgia Tech Research Institute"),
                company_link: String::from("https://gtri.gatech.edu/"),
                description: String::from(
                "
                I worked within a team of researchers trying to improve visualization techniques for \
                neural network training while working with Python using Tensforflow and PyTorch. \
                Then I was apart of the AGILE development process in order to create a web application \
                using React to display these visualizations in components, utilizing a Flask backend \
                powered with a MongoDB database.
                "),
                tags: vec![
                    String::from("JavaScript"),
                    String::from("Python"),
                    String::from("React"),
                    String::from("Flask"),
                    String::from("MongoDB"),
                    String::from("Neural Network"),
                    String::from("Data Visualization"),
                ],
            },
        ]
    }

}