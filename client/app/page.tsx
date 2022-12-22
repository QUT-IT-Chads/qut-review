import ReviewCard from './review/ReviewCard';
import { Review } from '../types/review/types';

const reviews: Array<Review> = [
    {
        unitCode: 'CAB431',
        unitName: 'Cloud Computing',
        reviewBody: 'Pretty alright, aye.',
        lastUpdated: Date.now()
    },
    {
        unitCode: 'CAB202',
        unitName: 'Microprocessor',
        reviewBody:
            'Felt more tailiered to engineering students but was still a good unit',
        lastUpdated: Date.now()
    },
    {
        unitCode: 'IFB104',
        unitName: 'Building IT Systems',
        reviewBody: "Easy shit I've done, such a piss take for a uni subject",
        lastUpdated: Date.now()
    }
];

export default function Page() {
    return (
        <div className="p-5">
            <div className="grid grid-rows-3 gap-8 lg:grid-cols-3">
                {reviews.map((review, key) => (
                    <ReviewCard key={key} review={review} />
                ))}
            </div>
        </div>
    );
}
