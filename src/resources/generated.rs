//! generated module
//!
//! Contains the generated impls we use. This code
//! is automatically generated from the openapi spec
//! and should not be changed manually. To update the
//! spec, use cargo make.
//!
//! It is possible more files are generated than are
//! listed as modules here. These are modules that
//! have not yet been exposed by the client.

#[path = "generated"]
pub mod core {
    pub mod account_application_authorized;
    pub mod account_application_deauthorized;
    pub mod account_external_account_created;
    pub mod account_external_account_deleted;
    pub mod account_external_account_updated;
    pub mod account_session;
    pub mod account_updated;
    pub mod address;
    pub mod api_errors;
    pub mod balance;
    pub mod balance_amount_by_source_type;
    pub mod balance_available;
    pub mod balance_transaction;
    pub mod billing_details;
    pub mod cash_balance;
    pub mod charge;
    pub mod charge_captured;
    pub mod charge_dispute_closed;
    pub mod charge_dispute_created;
    pub mod charge_dispute_funds_reinstated;
    pub mod charge_dispute_funds_withdrawn;
    pub mod charge_dispute_updated;
    pub mod charge_expired;
    pub mod charge_failed;
    pub mod charge_pending;
    pub mod charge_refund_updated;
    pub mod charge_refunded;
    pub mod charge_succeeded;
    pub mod charge_updated;
    pub mod connect_account_reference;
    pub mod custom_unit_amount;
    pub mod customer;
    pub mod customer_cash_balance_transaction;
    pub mod customer_cash_balance_transaction_created;
    pub mod customer_created;
    pub mod customer_deleted;
    pub mod customer_session;
    pub mod customer_updated;
    pub mod dispute;
    pub mod dispute_transaction_shipping_address;
    pub mod ephemeral_key;
    pub mod file;
    pub mod file_created;
    pub mod file_link;
    pub mod linked_account_options_us_bank_account;
    pub mod mandate;
    pub mod mandate_updated;
    pub mod payment_flows_amount_details_client_resource_tip;
    pub mod payment_flows_payment_intent_presentment_details;
    pub mod payment_flows_private_payment_methods_alipay;
    pub mod payment_intent;
    pub mod payment_intent_next_action_cashapp_handle_redirect_or_display_qr_code;
    pub mod payment_method_config_biz_payment_method_configuration_details;
    pub mod payment_method_details_card_installments_plan;
    pub mod payment_method_details_card_present;
    pub mod payment_method_details_card_present_offline;
    pub mod payment_method_details_card_wallet_apple_pay;
    pub mod payment_method_details_card_wallet_google_pay;
    pub mod payment_method_details_passthrough_card;
    pub mod payment_method_options_customer_balance_eu_bank_account;
    pub mod payment_method_options_us_bank_account_mandate_options;
    pub mod payout;
    pub mod price;
    pub mod product;
    pub mod radar_radar_options;
    pub mod refund;
    pub mod refund_created;
    pub mod refund_failed;
    pub mod refund_updated;
    pub mod reserve_transaction;
    pub mod setup_attempt;
    pub mod setup_intent;
    pub mod shipping;
    pub mod shipping_rate;
    pub mod tax_code;
    pub mod tax_deducted_at_source;
    pub mod test_helpers_test_clock;
    pub mod token;
    pub mod version;
}

#[path = "generated"]
pub mod payment {
    pub mod bank_account;
    pub mod card;
    pub mod payment_method;
    pub mod payment_method_acss_debit;
    pub mod payment_method_affirm;
    pub mod payment_method_afterpay_clearpay;
    pub mod payment_method_alma;
    pub mod payment_method_amazon_pay;
    pub mod payment_method_attached;
    pub mod payment_method_au_becs_debit;
    pub mod payment_method_automatically_updated;
    pub mod payment_method_bacs_debit;
    pub mod payment_method_bancontact;
    pub mod payment_method_billie;
    pub mod payment_method_blik;
    pub mod payment_method_boleto;
    pub mod payment_method_card;
    pub mod payment_method_card_present;
    pub mod payment_method_cashapp;
    pub mod payment_method_customer_balance;
    pub mod payment_method_detached;
    pub mod payment_method_eps;
    pub mod payment_method_fpx;
    pub mod payment_method_giropay;
    pub mod payment_method_grabpay;
    pub mod payment_method_ideal;
    pub mod payment_method_interac_present;
    pub mod payment_method_kakao_pay;
    pub mod payment_method_klarna;
    pub mod payment_method_konbini;
    pub mod payment_method_kr_card;
    pub mod payment_method_link;
    pub mod payment_method_mobilepay;
    pub mod payment_method_multibanco;
    pub mod payment_method_naver_pay;
    pub mod payment_method_nz_bank_account;
    pub mod payment_method_options_card_present_routing;
    pub mod payment_method_options_customer_balance_eu_bank_account;
    pub mod payment_method_options_us_bank_account_mandate_options;
    pub mod payment_method_oxxo;
    pub mod payment_method_p24;
    pub mod payment_method_pay_by_bank;
    pub mod payment_method_payco;
    pub mod payment_method_paynow;
    pub mod payment_method_paypal;
    pub mod payment_method_pix;
    pub mod payment_method_promptpay;
    pub mod payment_method_revolut_pay;
    pub mod payment_method_samsung_pay;
    pub mod payment_method_satispay;
    pub mod payment_method_sepa_debit;
    pub mod payment_method_sofort;
    pub mod payment_method_swish;
    pub mod payment_method_twint;
    pub mod payment_method_updated;
    pub mod payment_method_us_bank_account;
    pub mod payment_method_wechat_pay;
    pub mod payment_method_zip;
    pub mod source;
}

#[path = "generated"]
#[cfg(feature = "checkout")]
pub mod checkout {
    pub mod checkout_session;
    pub mod checkout_session_async_payment_failed;
    pub mod checkout_session_async_payment_succeeded;
    pub mod checkout_session_completed;
    pub mod checkout_session_expired;
    pub mod item;
    pub mod payment_link;
    pub mod payment_link_created;
    pub mod payment_link_updated;
}

#[path = "generated"]
#[cfg(feature = "tax-calculation")]
pub mod tax_calculation {
    pub mod tax_calculation;
    pub mod tax_calculation_line_item;
    pub mod tax_product_resource_customer_details;
}

#[path = "generated"]
#[cfg(feature = "billing")]
pub mod billing {
    pub mod billing_bill_resource_invoicing_lines_common_proration_details;
    pub mod billing_bill_resource_invoicing_pricing_pricing;
    pub mod billing_credit_balance_summary;
    pub mod billing_credit_balance_transaction;
    pub mod billing_credit_balance_transaction_created;
    pub mod billing_credit_grant;
    pub mod billing_credit_grant_created;
    pub mod billing_credit_grant_updated;
    pub mod billing_credit_grants_resource_amount;
    pub mod billing_portal_configuration;
    pub mod billing_portal_configuration_created;
    pub mod billing_portal_configuration_updated;
    pub mod billing_portal_session;
    pub mod billing_portal_session_created;
    pub mod coupon;
    pub mod coupon_created;
    pub mod coupon_deleted;
    pub mod coupon_updated;
    pub mod credit_note;
    pub mod credit_note_created;
    pub mod credit_note_line_item;
    pub mod credit_note_updated;
    pub mod credit_note_voided;
    pub mod customer_balance_transaction;
    pub mod discount;
    pub mod invoice;
    pub mod invoice_created;
    pub mod invoice_deleted;
    pub mod invoice_finalization_failed;
    pub mod invoice_finalized;
    pub mod invoice_marked_uncollectible;
    pub mod invoice_paid;
    pub mod invoice_payment;
    pub mod invoice_payment_action_required;
    pub mod invoice_payment_failed;
    pub mod invoice_payment_method_options_acss_debit;
    pub mod invoice_payment_method_options_bancontact;
    pub mod invoice_payment_method_options_customer_balance;
    pub mod invoice_payment_method_options_konbini;
    pub mod invoice_payment_method_options_sepa_debit;
    pub mod invoice_payment_method_options_us_bank_account;
    pub mod invoice_payment_succeeded;
    pub mod invoice_sent;
    pub mod invoice_setting_checkout_rendering_options;
    pub mod invoice_upcoming;
    pub mod invoice_updated;
    pub mod invoice_voided;
    pub mod invoiceitem;
    pub mod invoiceitem_created;
    pub mod invoiceitem_deleted;
    pub mod invoices_resource_shipping_cost;
    pub mod line_item;
    pub mod plan;
    pub mod plan_created;
    pub mod plan_deleted;
    pub mod plan_updated;
    pub mod promotion_code;
    pub mod quote;
    pub mod quotes_resource_total_details;
    pub mod subscription;
    pub mod subscription_item;
    pub mod subscription_schedule;
    pub mod subscriptions_trials_resource_trial_settings;
    pub mod tax_id;
    pub mod tax_product_resource_customer_details;
    pub mod tax_product_resource_ship_from_details;
    pub mod tax_rate;
    pub mod tax_rate_created;
    pub mod tax_rate_flat_amount;
    pub mod tax_rate_updated;
}

#[path = "generated"]
#[cfg(feature = "connect")]
pub mod connect {
    pub mod account;
    pub mod account_link;
    pub mod application;
    pub mod application_fee;
    pub mod connect_collection_transfer;
    pub mod fee_refund;
    pub mod login_link;
    pub mod person;
    pub mod topup;
    pub mod transfer;
    pub mod transfer_reversal;
}

#[path = "generated"]
#[cfg(feature = "fraud")]
pub mod fraud {
    pub mod review;
}

#[path = "generated"]
#[cfg(feature = "issuing")]
pub mod issuing {
    pub mod issuing_authorization;
    pub mod issuing_card;
    pub mod issuing_cardholder;
    pub mod issuing_dispute;
    pub mod issuing_personalization_design;
    pub mod issuing_personalization_design_activated;
    pub mod issuing_personalization_design_deactivated;
    pub mod issuing_personalization_design_rejected;
    pub mod issuing_personalization_design_updated;
    pub mod issuing_physical_bundle;
    pub mod issuing_token;
    pub mod issuing_transaction;
}

#[cfg(feature = "sigma")]
pub mod scheduled_query_run;

#[path = "generated"]
#[cfg(feature = "terminal")]
pub mod terminal {
    pub mod terminal_configuration;
    pub mod terminal_connection_token;
    pub mod terminal_location;
    pub mod terminal_reader;
}

#[cfg(feature = "events")]
pub mod event;

#[path = "generated"]
#[cfg(feature = "webhook-endpoints")]
pub mod webhook_endpoints {
    pub mod webhook_endpoint;
}

#[cfg(not(feature = "full"))]
pub mod placeholders;
