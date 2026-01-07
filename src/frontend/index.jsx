import React from 'react';
import ForgeReconciler, {Text, useProductContext} from "@forge/react"; // useProductContext hook for getting the context of the Atlassian app
import {requestJira} from "@forge/bridge"; // the Jira REST API


const App = () => {
    const context = useProductContext();

    const [comments, setComments] = React.useState([]);
    console.log(`Number of comments on this issue: ${comments?.length}`);

    // Jira issue call
    const fetchCommentsForIssue = async () => {
        // extracting issue id instead of getting from an input
        const issueId = context?.extension.issue.id;
        const response = await requestJira(`/rest/api/3/issue/${issueId}/comment`);
        const data = await response.json();
        return data?.comments;
    }

    React.useEffect(() => {
        if (context) {
            fetchCommentsForIssue().then(setComments)
        }
    }, [context]);

    return <Text>Number of comments: {comments?.length}</Text>
}

ForgeReconciler.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
)