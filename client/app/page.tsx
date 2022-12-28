import RecentCard from './(components)/(RecentCard)';
import Recent from 'types/Recent';

const recents: Array<Recent> = [
    {
        unitCode: 'CAB431',
        unitName: 'Cloud Computing',
        unitDescription: `With the explosion of information resources on the Web,
            social media and corporate intranets, there is an imminent
            need for advanced technologies to help people deal with big
            text data. There are many practical applications of Web search
            and text analysis in the areas such as classification of news
            stories, academic papers or medical records; spam or junk email
            filtering, understand customers opinion or behaviors through
            their feedback in online-systems or social media, customer service
            promotion etc. Therefore, it is urgent for IT developers, Web
            analysts, information management consultants, or Web development &
            support officers to understand NLP (Natural Language Processing)
            techniques, popular text processing models (such as Web search
            engine, information retrieval models); advanced text mining
            techniques (such as supervised methods for information
            filtering or classification and unsupervised method for topic
            modelling); and future directions in Web Intelligence.`,
        score: 5,
        reviewed: new Date()
    },
    {
        unitCode: 'CAB202',
        unitName: 'Microprocessors and Digital Systems',
        unitDescription: `This unit introduces you to the components inside a computer and
            how these components work together. The design and development of
            modern digital electronic systems requires a knowledge of the
            hardware  and software to program the system. This unit identifies
            design requirements and lets you develop embedded
            microcontroller-based system solutions. Practical laboratory
            exercises progressively expose features of a typical microprocessor;
            and explain how an embedded computer can interact with its environment.
            This provides a valuable foundation for further studies in areas such
            as robotics and networking.`,
        score: 5,
        reviewed: new Date()
    },
    {
        unitCode: 'IFB104',
        unitName: 'Building IT Systems',
        unitDescription: `This unit provides a hands-on introduction to computer
            programming for students with no prior coding experience at all. It
            introduces the basic principles of programming in a typical imperative
            language, including expressions, assignment, functions, choice and
            iteration. It then shows how to use Application Programming Interfaces to
            complete common Information Technology tasks such as querying databases,
            creating user interfaces, and searching for patterns in large datasets.
            The emphasis is on developing skills through practice, so the unit includes
            numerous coding exercises and assignments, using a simple scripting language
            and code development environment. The unit establishes a foundation for
            later subjects that teach large-scale software development using
            industrial-strength programming languages.`,
        score: 5,
        reviewed: new Date()
    }
];

export default function Page() {
    return (
        <div className="p-5">
            <div className="grid grid-rows-3 gap-8 lg:grid-cols-3">
                {recents.map((recent, key) => (
                    <RecentCard key={key} recent={recent} />
                ))}
            </div>
        </div>);
}
