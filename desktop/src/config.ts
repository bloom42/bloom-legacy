const WEBSITE_DOMAIN = 'bloom.sh';

// eslint disables are due to __static being marked as not declared
export default {
  APP_ID: 'com.bloom42.bloom',
  TRAY_ICON: 'assets/icons/tray.png',
  WINDOW_TITLE: 'Bloom',
  WINDOW_ICON: 'bloom_1024.png',
  WINDOW_URL: 'http://localhost:8080',
  WINDOW_DEFAULT_WIDTH: 900,
  WINDOW_DEFAULT_HEIGHT: 700,
  WINDOW_MIN_WIDTH: 600,
  WINDOW_MIN_HEIGHT: 600,
  WEBSITE_URL: `https://${WEBSITE_DOMAIN}`,
  TERMS_URL: `https://${WEBSITE_DOMAIN}/terms`,
  PRIVACY_URL: `https://${WEBSITE_DOMAIN}/privacy`,
  HELP_URL: `https://help.${WEBSITE_DOMAIN}`,
};
