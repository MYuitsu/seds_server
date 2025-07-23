export const formatterDate = new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
    timeZoneName: "short",
});

export const formatterTime = new Intl.DateTimeFormat("en-US", {
    hour: "2-digit",
    minute: "2-digit",
    hour12: true,
});

export const formatterPeriod = (start: string, end: string) => {
    const startDate = new Date(start);
    const endDate = new Date(end);
    const date = formatterDate.format(startDate);
    const startTime = formatterTime.format(startDate);
    const endTime = formatterTime.format(endDate);
    return `Period: ${date} (${startTime} - ${endTime})`;
}

export const formatterDateTime = (timestamp: string) => {
    const date = new Date(timestamp);
    return `${formatterDate.format(date)}, ${formatterTime.format(date)}`;
}
