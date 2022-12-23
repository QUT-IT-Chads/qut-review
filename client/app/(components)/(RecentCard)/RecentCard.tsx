import React from 'react';
import Recent from 'types/Recent';

interface recentProps {
    recent: Recent;
}

const RecentCard = (props: recentProps) => {
    const { unitCode, unitName, unitDescription, score, reviewed } =
        props.recent;

    const MAX_LENGTH = 485;
    let fixedDescription = '';

    if (unitDescription.length > MAX_LENGTH) {
        fixedDescription = unitDescription.substring(0, MAX_LENGTH) + '...';
    } else {
        fixedDescription = unitDescription;
    }

    return (
        <div
            className="
                flex h-full w-full
                drop-shadow-[0px_0px_5px_rgba(1,1,1,0.50)]
                hover:scale-105 hover:transition hover:ease-in-out
            "
        >
            <div className="rounded-l-md bg-[#245AAC] px-2" />
            <div
                className="
                    flex w-full flex-col justify-between
                    rounded-r-md bg-white py-2 px-3
                    hover:transition hover:ease-in-out
                "
            >
                <div id="title">
                    <h1 className="text-start text-xl">{unitCode}</h1>
                    <h2 className="text-start text-base">{unitName}</h2>
                </div>
                {/* Star needs to be inline with unit details*/}
                <div>{score}</div>

                <div id="paragraph" className="text-start text-sm">
                    {fixedDescription}
                </div>

                <span className="text-end text-sm">
                    {reviewed.toLocaleTimeString()}, {reviewed.toDateString()}
                </span>
            </div>
        </div>
    );
};

export default RecentCard;
