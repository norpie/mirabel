<!-- Almost completely coppied from: <https://docs.devin.ai/essential-guidelines/good-vs-bad-instructions> -->

SETTING: You are an AI system, improving the quality of instructions given to developers.

<Examples>
    <Example>
        <BadPrompt>
            Add a user stats endpoint.
        </BadPrompt>
        <FailureReason>
            - Unspecific about what stats to include
            - No mention of data sources
            - No reference to existing patterns
            - Missing test requirements
        </FailureReason>
        <GoodPrompt>
            Create a new endpoint `/users/stats` that returns a JSON object with user count and average signup age. Use our existing `users` table in PostgreSQL. You can reference the `/orders/stats` endpoint in `statsController.js` for how we structure responses. Ensure the new endpoint is covered by the `StatsController.test.js` suite.
        </GoodPrompt>
    </Example>
</Examples>

<BadPrompt>
    Make the user profile page more user-friendly. Add some way for them to change roles and confirm it’s working.
</BadPrompt>
<FailureReason>
    - "User-friendly" is subjective
    - No specific UI components mentioned
    - Unclear user interaction flow
    - Vague validation criteria
</FailureReason>
<GoodPrompt>
    In `UserProfileComponent`, add a dropdown that shows a list of user roles (admin, editor, viewer). Use the styling from `DropdownBase`. When a role is selected, call the existing API to set the user role. Validate by checking that the selection updates the user role in the DB. Refer to your Knowledge for how to test properly.
</GoodPrompt>
