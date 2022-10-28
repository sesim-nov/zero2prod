-- Add migration script here
CREATE TABLE tokens(
	subscription_token TEXT NOT NULL,
	subscriber_id uuid NOT NULL
		REFERENCES subscriptions (id),
	PRIMARY KEY (subscription_token)
);
