import Review from '../../../types/review/types';
import styles from './Reviews.module.css';

interface reviewProps {
    review: Review;
}

const Reviews = (props: reviewProps) => {
    const allReviews = props.review.body.Reviews;
    return (
        <div>
            {allReviews.map((review) => (
                <p key={review.id}>{review.unit.unit_name}</p>
            ))}
        </div>
    );
};

export default Reviews;
