// import { useRouter } from 'next/navigation';

// export async function getStaticProps(context) {
//     const res = await fetch(`${process.env.API_URL}`);

//     return {
//         props: {
//             code: {}
//         }
//     };
// }

export interface Slug {
    params: {
        code: string;
    };
}

export default function Page({ params }: Slug) {
    console.log(params);

    return <h1>Hi there</h1>;
}
