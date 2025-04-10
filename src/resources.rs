//! resources module
//!
//! This module exposes various elements of the
//! stripe api depending on the features exposed.
//!
//! Some of these modules are hand-written, and
//! some are generated.

mod currency;
#[allow(clippy::module_inception)]
#[allow(clippy::new_without_default)]
pub mod generated;
mod types;

#[path = "resources"]
mod core {
    pub mod account_ext;
    pub mod balance_ext;
    pub mod balance_transaction_ext;
    pub mod charge_ext;
    pub mod customer_ext;
    pub mod payment_intent_ext;
    pub mod payment_source;
    pub mod payout_ext;
    pub mod placeholders;
    pub mod setup_intent_ext;
    pub mod test_clock_ext;
    pub mod token_ext;
    pub mod transfer_reversal_ext;
}

#[path = "resources"]
mod payment {
    pub mod bank_account_ext;
    pub mod card;
    pub mod payment_method_ext;
    pub mod source_ext;
}

#[cfg(feature = "events")]
mod webhook_events;

#[path = "resources"]
#[cfg(feature = "billing")]
mod billing {
    pub mod credit_note_ext;
    pub mod customer_balance_transaction_ext;
    pub mod invoice_ext;
    pub mod line_item_ext;
    pub mod subscription_ext;
    // pub mod usage_record_ext;
}

#[path = "resources"]
#[cfg(feature = "products")]
mod products {
    pub mod price_ext;
    pub mod product_ext;
    pub mod promotion_code_ext;
}

#[path = "resources"]
#[cfg(feature = "checkout")]
mod checkout {
    pub mod checkout_session_ext;
}

#[path = "resources"]
#[cfg(feature = "connect")]
mod connect {
    pub mod login_links_ext;
}

#[path = "resources"]
#[cfg(feature = "fraud")]
mod fraud {
    pub mod review_ext;
}

#[path = "resources"]
#[cfg(feature = "issuing")]
mod issuing {
    pub mod issuing_authorization_ext;
    pub mod issuing_card_ext;
    pub mod issuing_dispute_ext;
    pub mod issuing_merchant_data;
    pub mod issuing_transaction_ext;
}

#[path = "resources"]
#[cfg(feature = "orders")]
mod orders {
    pub mod order_ext;
}

#[path = "resources"]
#[cfg(feature = "webhook-endpoints")]
mod webhook_endpoints {
    pub mod webhook_endpoint_ext;
}

#[rustfmt::skip]
pub use {
    currency::*,
    types::*,

    self::core::{
        account_ext::*,
        balance_transaction_ext::*,
        charge_ext::*,
        transfer_reversal_ext::*,
        customer_ext::*,
        payment_intent_ext::*,
        payment_source::*,
        placeholders::*,
        payout_ext::*,
        test_clock_ext::*,
        token_ext::*,
        setup_intent_ext::*,
    },
    generated::core::{
        address::*,
        balance::*,
        balance_amount_by_source_type::*,
        balance_transaction::*,
        billing_details::*,
        charge::*,
        connect_account_reference::*,
        customer::*,
        customer_session::*,
        custom_unit_amount::*,
        cash_balance::*,
        dispute::*,
        dispute_transaction_shipping_address::*,
        ephemeral_key::*,
        file::*,
        file_link::*,
        mandate::*,
        payment_intent::*,
        payment_intent_next_action_cashapp_handle_redirect_or_display_qr_code::*,
        linked_account_options_us_bank_account::*,
        payment_flows_amount_details_client_resource_tip::*,
        payment_flows_payment_intent_presentment_details::*,
        payment_flows_private_payment_methods_alipay::*,
        payment_method_details_card_installments_plan::*,
        payment_method_details_card_present_offline::*,
        payment_method_details_card_present::*,
        payment_method_details_card_wallet_apple_pay::*,
        payment_method_details_card_wallet_google_pay::*,
        payment_method_details_passthrough_card::*,
        payment_method_config_biz_payment_method_configuration_details::*,
        payout::*,
        price::*,
        product::*,
        radar_radar_options::*,
        refund::*,
        reserve_transaction::*,
        setup_attempt::*,
        setup_intent::*,
        shipping::*,
        shipping_rate::*,
        tax_code::*,
        tax_deducted_at_source::*,
        test_helpers_test_clock::*,
        token::*,
        api_errors::*,
    },

    payment::{
        bank_account_ext::*,
        card::*,
        payment_method_ext::*,
        source_ext::*
    },
    generated::payment::{
        card::*,
        bank_account::*,
        payment_method::*,
        payment_method_acss_debit::*,
        payment_method_affirm::*,
        payment_method_afterpay_clearpay::*,
        payment_method_alma::*,
        payment_method_amazon_pay::*,
        payment_method_attached::*,
        payment_method_au_becs_debit::*,
        payment_method_automatically_updated::*,
        payment_method_bacs_debit::*,
        payment_method_bancontact::*,
        payment_method_billie::*,
        payment_method_blik::*,
        payment_method_boleto::*,
        payment_method_card_present::*,
        payment_method_card::*,
        payment_method_cashapp::*,
        payment_method_customer_balance::*,
        payment_method_detached::*,
        payment_method_eps::*,
        payment_method_fpx::*,
        payment_method_giropay::*,
        payment_method_grabpay::*,
        payment_method_ideal::*,
        payment_method_interac_present::*,
        payment_method_kakao_pay::*,
        payment_method_klarna::*,
        payment_method_konbini::*,
        payment_method_kr_card::*,
        payment_method_link::*,
        payment_method_mobilepay::*,
        payment_method_multibanco::*,
        payment_method_naver_pay::*,
        payment_method_nz_bank_account::*,
        payment_method_options_customer_balance_eu_bank_account::*,
        payment_method_options_us_bank_account_mandate_options::*,
        payment_method_options_card_present_routing::*,
        payment_method_oxxo::*,
        payment_method_p24::*,
        payment_method_pay_by_bank::*,
        payment_method_payco::*,
        payment_method_paynow::*,
        payment_method_paypal::*,
        payment_method_pix::*,
        payment_method_promptpay::*,
        payment_method_revolut_pay::*,
        payment_method_samsung_pay::*,
        payment_method_satispay::*,
        payment_method_sepa_debit::*,
        payment_method_sofort::*,
        payment_method_swish::*,
        payment_method_twint::*,
        payment_method_updated::*,
        payment_method_us_bank_account::*,
        payment_method_wechat_pay::*,
        payment_method_zip::*,
        source::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "events")]
pub use {
    webhook_events::*,
    webhook_events::NotificationEventData,
    generated::event::*,
};

#[rustfmt::skip]
#[cfg(feature = "checkout")]
pub use {
    checkout::checkout_session_ext::*,
    generated::checkout::{
        checkout_session::*,
        payment_link::*,
        item::*
    },
};

#[rustfmt::skip]
#[cfg(feature = "products")]
pub use {
    products::{
        product_ext::*,
        price_ext::*,
        promotion_code_ext::*,
    }
};

#[rustfmt::skip]
#[cfg(feature = "billing")]
pub use {
    billing::{
        customer_balance_transaction_ext::*,
        invoice_ext::*,
        line_item_ext::*,
        subscription_ext::*,
        // usage_record_ext::*,
    },
    generated::billing::{
        billing_bill_resource_invoicing_lines_common_proration_details::*,
        billing_bill_resource_invoicing_pricing_pricing::*,
        billing_credit_balance_summary::*,
        billing_credit_balance_transaction_created::*,
        billing_credit_balance_transaction::*,
        billing_credit_grant_created::*,
        billing_credit_grant_updated::*,
        billing_credit_grant::*,
        billing_credit_grants_resource_amount::*,
        billing_portal_session::*,
        billing_portal_configuration::*,
        coupon::*,
        credit_note::*,
        credit_note_line_item::*,
        customer_balance_transaction::*,
        discount::*,
        invoice_payment_action_required::*,
        invoice_payment_failed::*,
        invoice_payment_method_options_acss_debit::*,
        invoice_payment_method_options_bancontact::*,
        invoice_payment_method_options_customer_balance::*,
        invoice_payment_method_options_konbini::*,
        invoice_payment_method_options_sepa_debit::*,
        invoice_payment_method_options_us_bank_account::*,
        invoice_payment_succeeded::*,
        invoice_payment::*,
        invoice_setting_checkout_rendering_options::*,
        invoiceitem::*,
        invoice::*,
        invoices_resource_shipping_cost::*,
        line_item::*,
        plan::*,
        plan::PlanInterval,
        promotion_code::*,
        quote::*,
        quotes_resource_total_details::*,
        subscription_item::*,
        subscription_item::PlanInterval as SubscriptionItemInterval,
        subscription_item::SubscriptionItemPriceDataRecurring as SubscriptionItemPriceDataRecurring,
        subscription_item::SubscriptionItemPriceData as SubscriptionItemPriceData,
        subscription_item::SubscriptionPaymentBehavior as SubscriptionItemPaymentBehavior,
        // need to import this afterwards so that the SubscriptionItemPriceDataRecurring
        // isn't silently ignored
        subscription::*,
        subscriptions_trials_resource_trial_settings::*,
        subscription::PlanInterval as SubscriptionInterval,
        subscription::SubscriptionItemPriceDataRecurring as SubscriptionPriceDataRecurring,
        subscription::SubscriptionItemPriceData as SubscriptionPriceData,
        subscription::SubscriptionPaymentBehavior as SubscriptionPaymentBehavior,
        subscription_schedule::*,
        tax_id::*,
        tax_product_resource_ship_from_details::*,
        tax_rate_created::*,
        tax_rate_flat_amount::*,
        tax_rate_updated::*,
        tax_rate::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "tax-calculation")]
pub use {
    generated::tax_calculation::{
        tax_calculation::*,
        tax_calculation_line_item::*,
        tax_product_resource_customer_details::*,
    }
};

#[rustfmt::skip]
#[cfg(feature = "connect")]
pub use {
    connect::{
        login_links_ext::*,
    },
    generated::connect::{
        account_link::*,
        account::*,
        application::*,
        application_fee::*,
        connect_collection_transfer::*,
        fee_refund::*,
        login_link::*,
        person::*,
        topup::*,
        transfer::*,
        transfer_reversal::*,
    }
};

#[rustfmt::skip]
#[cfg(feature = "fraud")]
pub use {
    fraud::review_ext::*,
    generated::fraud::review::*
};

#[rustfmt::skip]
#[cfg(feature = "issuing")]
pub use {
    issuing::{
        issuing_authorization_ext::*,
        issuing_card_ext::*,
        issuing_dispute_ext::*,
        issuing_merchant_data::*,
        issuing_transaction_ext::*,
    },
    generated::issuing::{
        issuing_authorization::*,
        issuing_card::*,
        issuing_token::*,
        issuing_cardholder::*,
        issuing_dispute::*,
        issuing_personalization_design_activated::*,
        issuing_personalization_design_deactivated::*,
        issuing_personalization_design_rejected::*,
        issuing_personalization_design_updated::*,
        issuing_personalization_design::*,
        issuing_physical_bundle::*,
        issuing_transaction::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "orders")]
pub use {
    orders::order_ext::*,
};

#[rustfmt::skip]
#[cfg(feature = "sigma")]
pub use {
    generated::scheduled_query_run::*,
};

#[rustfmt::skip]
#[cfg(feature = "terminal")]
pub use {
    generated::terminal::{
        terminal_configuration::*,
        terminal_connection_token::*,
        terminal_location::*,
        terminal_reader::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "webhook-endpoints")]
pub use {
    webhook_endpoints::webhook_endpoint_ext::*,
    generated::webhook_endpoints::webhook_endpoint::*,
};

#[cfg(not(feature = "full"))]
pub use generated::placeholders::*;

/// this struct is just a stub for code not using the "connect" feature
/// see https://github.com/arlyon/async-stripe/issues/49 for more context
/// if there are more features that requires a fully fledged CompanyParams
/// we probably need to update the code generation and move to a shared place
#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CompanyParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}

/// this struct is just a stub for code not using the "connect" feature
/// see https://github.com/arlyon/async-stripe/issues/49 for more context
/// if there are more features that requires a fully fledged PersonParams
/// we probably need to update the code generation and move to a shared place
#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PersonParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}
