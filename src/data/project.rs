#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub image_path: String,
    pub is_public: bool,
    pub github_link: String,
}


impl Project {
    pub fn data() -> Vec<Project> {
        vec![
            Project {
                title: "Advising Pathways".to_string(),
                description: "Worked on a web application developed for the College of Computing advising department at Georgia Tech. It facilitates the exploration of curriculum requirements and options through surveys, gamified walkthroughs, and detailed advising information.".to_string(),
                image_path: "img/advising-pathways.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/AdvisingPathways".to_string(),
            },
            Project {
                title: "Chat Room".to_string(),
                description: "Created a server and client with the option of UDP/TCP that is ran through the command line.".to_string(),
                image_path: "img/chatroom.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/ChatRoom".to_string(),
            },
            Project {
                title: "Chess Tactic Classification".to_string(),
                description: "Trained AI to classify the strategies used in a sequence of moves recommended by Chess Engines by utilizing the chess puzzle database by lichess.".to_string(),
                image_path: "img/chess-tactic-classification.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/ChessTacticClassification".to_string(),
            },
            Project {
                title: "Climate change prediction".to_string(),
                description: "Developed models to predict future surface temperatures for various regions around the world and display that data on an interactive globe visualization.".to_string(),
                image_path: "img/climate-change-prediction.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/climate-change-prediction".to_string(),
            },
            Project {
                title: "Collaborative Filtering".to_string(),
                description: "Performed SVD factorization on an Anime recommendations database to give suggestions of content for new users.".to_string(),
                image_path: "img/collaborative-filtering.jpeg".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "D3.js Visualizations".to_string(),
                description: "Created visualizations displaying various properties of a Board Game dataset including a Graph for connecting similarities, line charts to show ratings, and a choropleth map to show average rating per region.".to_string(),
                image_path: "img/d3.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "DS Labs".to_string(),
                description: "Design and implement various distributed systems strategies for database replication in a simulated environment. These strategies included Primary-Backup, Multi-Paxos, and a strategy resembling Google Spanner.".to_string(),
                image_path: "img/dslabs.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "FarmGuardians".to_string(),
                description: "Created a farm simulation game that featured an interactive ui and simple visuals.".to_string(),
                image_path: "img/farm-guardians.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/FarmGuardians".to_string(),
            },
            Project {
                title: "FlexBros".to_string(),
                description: "Developed a web application for multiple platforms to build workout routines and monitor progress. It featured a workout calendar to display routines with the ability to log daily performance.".to_string(),
                image_path: "img/flex-bros.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/FlexBros".to_string(),
            },
            Project {
                title: "GBA Game".to_string(),
                description: "Using a GBA emulator created a simple pong game.".to_string(),
                image_path: "img/pong.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/GBA-Pong".to_string(),
            },
            Project {
                title: "Feature Extraction for images".to_string(),
                description: "Implemented the harris corner detection algorithm.".to_string(),
                image_path: "img/harris-sift.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Image Compression using ML".to_string(),
                description: "Performed Gaussian Mixture Modeling, Kmeans, and PCA methods of analyzing data and showed their effectiveness by compressing images.".to_string(),
                image_path: "img/image-compression.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Image Description for the Visually Impaired".to_string(),
                description: "Created an novel approach to create image descriptions by using image captioning model output fed to LLMs asking questions to VQA models to generate paragraph length descriptions.".to_string(),
                image_path: "img/image-description-for-visually-impaired.png".to_string(),
                is_public: true,
                github_link: "https://github.com/kevin-pietruszka/image-description-for-visually-impaired".to_string(),
            },
            Project {
                title: "Implicit Surfaces".to_string(),
                description: "Visualized 3D models using implicit surface drawing techniques, complete with blobby shapes of standard primitives. It also allows for the twist, taper and bend deformation techniques to improve the looks of various different morphed objects.".to_string(),
                image_path: "img/implicit-surfaces.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Logic Gate Computer Simulation".to_string(),
                description: "Used basic logic gates to work up to a simple computer using a circuit simulator. The machine includes a simple bus and ram with the core components of a pc, regfile, and alu.".to_string(),
                image_path: "img/lc3.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Machine Learning in Network Traffic".to_string(),
                description: "Trained a PAYL model on example network traffic data and tested on normal and attack payloads. Employed a polymorphic blending technique to show the limitations of such methods.".to_string(),
                image_path: "img/payl.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Malware Analysis".to_string(),
                description: "Used sample malware in a VM environment to do network behavioral analysis to gain information on the control flow graph and symbolic execution.".to_string(),
                image_path: "img/malware.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Memory Page Simulator".to_string(),
                description: "Simulated virtual memory complete with a Paging implementation that included Page Eviction and Replacement policies.".to_string(),
                image_path: "img/vm.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Mesh Manipulation".to_string(),
                description: "Manipulated polygon meshes by applying Loop or Butterfly subdivision techniques. It adds surface noise and performs mesh smoothing, with the ability to toggle between flat and smooth shading.".to_string(),
                image_path: "img/mesh-manip.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Mini Internet".to_string(),
                description: "Participated in an internet connectivity simulation where each member controlled one Autonomous System that had to be inter connected with OSPF and iBGP protocols to then connect to external ASes through a BGP configuration.".to_string(),
                image_path: "img/mini-internet.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Network Monitoring".to_string(),
                description: "Analyzed four attack scenarios on an AWS network topology, targeting DDoS, brute force, web attacks, and botnets. Using provided sample traffic, I identified patterns indicative to create rules preventing each attack type.".to_string(),
                image_path: "img/wireshark.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Neural Network for Diabetes".to_string(),
                description: "Trained a neural network where I implemented the hidden layers and gradient descent to try to predict if a sample could be at risk of diabetes.".to_string(),
                image_path: "img/nn.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Pacman AI".to_string(),
                description: "Built various AI methods to play pacman that include BFS, DFS, UCS, A*, QLearning, and Markov Decision processes.".to_string(),
                image_path: "img/pacman.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Penetration Testing".to_string(),
                description: "Attacked a simulated CGI program by testing for open ports and using a reverse shell exploit to gain escalated privilege.".to_string(),
                image_path: "img/pen-testing.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Process Scheduler".to_string(),
                description: "Simulated an OS scheduling multi-threaded processes with an option to choose from the following algorithms: First Come First Serve, Shortest Job First, Round-Robin, Shortest Remaining Time First.".to_string(),
                image_path: "img/process-scheduler.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Projection Matrix and Fundamental Matrix Estimation with RANSAC".to_string(),
                description: "Implemented Project Matrix reconstruction given multiple images from a scene using RANSAC and then used the calculated matrix to calculate the epipolar lines between the images to match feature locations.".to_string(),
                image_path: "img/ransac.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Random Forest Classifying".to_string(),
                description: "Analyzed a set of over a hundred mushroom species and trained a random forest model to perform classification on an unknown mushroom sample.".to_string(),
                image_path: "img/random-forest.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Ray tracer".to_string(),
                description: "Created a distribution ray tracer with advanced shading techniques such as highlight and reflection alongside the special effects of soft shadows, motion blur, depth-of-field, and glossy reflection. The rendering is sped up with a Bounding Volume Hierarchy.".to_string(),
                image_path: "img/ray-tracer.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "RTP".to_string(),
                description: "Developed code to implement a Real-Time Transport Protocol using threads with a packet queue system. A client receives messages from a server using a packetization algorithm.".to_string(),
                image_path: "img/rtp.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Scene Recognition with CNNs and Bag-of-Words".to_string(),
                description: "Tried to classify a given scene of a various room or environment using a bag of words with sift feature output, and then tried using a Convolutional Neural Network for classification.".to_string(),
                image_path: "img/scene-recognition.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Simulated L2 Cache with DRAM".to_string(),
                description: "Simulated a multi-core processor utilizing an L2 cache with configurable eviction and write-back policies. The write-back operations are handled by a DRAM simulator that emulates pre-charging and row buffer behavior.".to_string(),
                image_path: "img/memory-sim.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Simulated Out-of-Order processor with cache and error flushing".to_string(),
                description: "Simulated a 7 stage processor with a Reorder Buffer and a Register Alias Table for out of order processing, and it interacts with a basic L1 cache.".to_string(),
                image_path: "img/ooo-processor.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Simulated Super Scalar with Branch Prediction".to_string(),
                description: "Tested the efficiency of a N wide superscalar processor with 5 basic stages while effectively managing dependencies with pipeline bubbles and flushes with branch predictions utilizing a gshare, gselect, or tournament predictor.".to_string(),
                image_path: "img/branch-prediction.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "Spark for Nyc taxi data analysis".to_string(),
                description: "Analyzed NYC taxi trip data to find trips that were least cost effective, the most popular, pricing differences between taxis, etc. Code was deployed on various cloud platforms like CGP, AWS, and Databricks for processing with PySpark.".to_string(),
                image_path: "img/taxi.jpg".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "SQL Schema for Travel Reservations Service".to_string(),
                description: "Developed an Entity-Relationship Diagram and SQL tables to manage travel reservations data combined with stored procedures for efficiency.".to_string(),
                image_path: "img/sql.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
            Project {
                title: "XSS".to_string(),
                description: "Attacked a simulated web application to perform reflective cross site scripting that is persistent across multiple sessions, and a stored cross site scripting attack on potential site admins.".to_string(),
                image_path: "img/xss.png".to_string(),
                is_public: false,
                github_link: "".to_string(),
            },
        ]
    }
}