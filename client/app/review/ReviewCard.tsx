import React from 'react';
import { Review } from '../../types/review/types';

interface reviewProps {
    review: Review;
}

const ReviewCard = (props: reviewProps) => {
    const { unitCode, unitName, reviewBody, lastUpdated } = props.review;

    return (
        <div className="flex h-full w-full hover:scale-105 hover:transition hover:ease-in-out">
            <div className="rounded-l-md bg-[#245AAC] px-2" />
            <div className="flex w-full flex-col justify-between rounded-r-md bg-white px-5 py-2 drop-shadow-[0_0px_5px_rgba(0,0,0,0.15)]">
                <div id="title">
                    <h1 className="text-start text-xl">{unitCode}</h1>
                    <h2 className="text-start text-base">{unitName}</h2>
                </div>

                <div id="paragraph" className="text-start text-sm">
                    {reviewBody}
                </div>

                <span className="text-end text-sm">{lastUpdated}</span>
            </div>
        </div>
    );
};

export default ReviewCard;
