@click_parallel_counts_2
Feature: Click Parallel Count (2)

    Background:

        Given I see the app

    Scenario Outline: Should increase the first and second counts

        Given I select the mode <Mode>
        And I select the component Parallel
        When I click the second count 3 times
        Then I see the first count is 3
        And I see the second count is 3

        Examples:
            | Mode         |
            | Out-of-Order |
            | In-Order     |
            | Async        |
