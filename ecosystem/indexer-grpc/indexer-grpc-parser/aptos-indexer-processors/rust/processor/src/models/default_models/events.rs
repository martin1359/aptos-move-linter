// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::extra_unused_lifetimes)]
use super::transactions::{Transaction, TransactionQuery};
use crate::{schema::events, utils::util::standardize_address};
use aptos_indexer_protos::transaction::v1::Event as EventPB;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Associations, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(belongs_to(Transaction, foreign_key = transaction_version))]
#[diesel(primary_key(account_address, creation_number, sequence_number))]
#[diesel(table_name = events)]
pub struct Event {
    pub sequence_number: i64,
    pub creation_number: i64,
    pub account_address: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub type_: String,
    pub data: serde_json::Value,
    pub event_index: Option<i64>,
}

/// Need a separate struct for queryable because we don't want to define the inserted_at column (letting DB fill)
#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[diesel(belongs_to(TransactionQuery, foreign_key = transaction_version))]
#[diesel(primary_key(account_address, creation_number, sequence_number))]
#[diesel(table_name = events)]
pub struct EventQuery {
    pub sequence_number: i64,
    pub creation_number: i64,
    pub account_address: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub type_: String,
    pub data: serde_json::Value,
    pub inserted_at: chrono::NaiveDateTime,
    pub event_index: Option<i64>,
}

impl Event {
    pub fn from_event(
        event: &EventPB,
        transaction_version: i64,
        transaction_block_height: i64,
        event_index: i64,
    ) -> Self {
        Event {
            account_address: standardize_address(
                event.key.as_ref().unwrap().account_address.as_str(),
            ),
            creation_number: event.key.as_ref().unwrap().creation_number as i64,
            sequence_number: event.sequence_number as i64,
            transaction_version,
            transaction_block_height,
            type_: event.type_str.clone(),
            data: serde_json::from_str(event.data.as_str()).unwrap(),
            event_index: Some(event_index),
        }
    }

    pub fn from_events(
        events: &[EventPB],
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Vec<Self> {
        events
            .iter()
            .enumerate()
            .map(|(index, event)| {
                Self::from_event(
                    event,
                    transaction_version,
                    transaction_block_height,
                    index as i64,
                )
            })
            .collect::<Vec<EventModel>>()
    }
}

// Prevent conflicts with other things named `Event`
pub type EventModel = Event;
