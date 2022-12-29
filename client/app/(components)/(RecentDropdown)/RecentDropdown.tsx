import React from 'react';
import Recent from 'types/Recent';

interface recentProps {
    recent: Recent;
}

const RecentDropdown = () => {//props: recentProps) => {
    // const { unitCode, unitName, unitDescription, score, reviewed } = props.recent;

    return (
        <div className='flex h-full w-full drop-shadow-[0px_0px_5px_rgba(1,1,1,0.50)]'>
            <div className='flex w-full flex-col justify-between rounded-l-md bg-[#3287FD] py-2 px-3 hover:transition hover:ease-in-out'>
                <div id="title">
                    {/*<h1 className="text-start text-xl">{unitCode}</h1>*/}
                    <h1 className="text-start text-xl">AAA000</h1>
                    {/*<h2 className="text-start text-base">{unitName}</h2>*/}
                    <h2 className="text-start text-base">Title of the Unit</h2>
                </div>
                {/* Star needs to be inline with unit details*/}
                <div>{/*{score}*/}</div>

                <div id="paragraph" className="text-start text-sm">
                    {/*{fixedDescription}*/}
                </div>

                <span className="text-end text-sm">
                    {/*{reviewed.toLocaleTimeString()}, {reviewed.toDateString()}*/}
                </span>
            </div>
            <div className="rounded-r-md bg-[#3287FD] px-2" />
        </div>
    );
};

export default RecentDropdown;
