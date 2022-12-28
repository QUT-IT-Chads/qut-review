import Reviews from './(Reviews)';
import Stats from './(Stats)';

const getReview = async () => {
    const res = await fetch(`http://127.0.0.1:8000/api/review`);
    return res.json();
};

const Page = async () => {
    const reviewData = getReview();
    const [review] = await Promise.all([reviewData]);
    return (
        <div>
            <Stats />
            <Reviews review={review} />
        </div>
    );
};

export default Page;
