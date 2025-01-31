#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;

#[component]
pub fn TransparencyIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let color: ColorTheme = use_context();

    rsx! {
        svg {
            "viewBox": "0 0 151 150",
            xmlns: "http://www.w3.org/2000/svg",
            height: "150",
            width: "151",
            fill: "none",
            ..attributes,
            path {
                fill: "{color.icon.primary}",
                d: "M14.5625 49.8047V107.496L70.8125 140.625V83.2031L14.5625 49.8047ZM80.1875 140.625L136.438 107.496V49.8047L80.1875 83.2031V140.625ZM131.75 42.1875L75.5 9.375L19.25 42.1875L75.5 75L131.75 42.1875Z",
            }
        }
    }
}

#[component]
pub fn FairenessIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color: ColorTheme = use_context();
    rsx! {

        svg {
            height: "120",
            xmlns: "http://www.w3.org/2000/svg",
            width: "151",
            "viewBox": "0 0 151 120",
            fill: "none",
            path {
                d: "M128 58.125V58.8984L144.336 42.5625C149.469 37.4297 149.469 29.1328 144.336 24L126.031 5.71875C120.898 0.585937 112.602 0.585937 107.469 5.71875L98.0703 15.1172C97.4375 15.0469 96.7812 15 96.125 15H69.875C61.1797 15 54.0312 21.5625 53.0938 30H53V58.125C53 63.3047 57.1953 67.5 62.375 67.5C67.5547 67.5 71.75 63.3047 71.75 58.125V37.5H109.25C119.609 37.5 128 45.8906 128 56.25V58.125ZM79.25 45V58.125C79.25 67.4531 71.7031 75 62.375 75C53.0469 75 45.5 67.4531 45.5 58.125V30.3281C37.0859 31.7812 30.0781 37.8984 27.6875 46.3125L23.8203 59.8125L6.66406 76.9688C1.53125 82.1016 1.53125 90.3984 6.66406 95.5312L24.9688 113.836C30.1016 118.969 38.3984 118.969 43.5312 113.836L52.3672 105C52.5781 105 52.7891 105.023 53 105.023H90.5C96.7109 105.023 101.75 99.9844 101.75 93.7734C101.75 92.4609 101.516 91.1953 101.117 90.0234H101.75C107.961 90.0234 113 84.9844 113 78.7734C113 75.7734 111.828 73.0547 109.906 71.0391C115.93 69.8672 120.477 64.5703 120.5 58.1953V58.1016C120.477 50.8828 114.617 45.0234 107.375 45.0234H79.25V45Z",
                fill: "{color.icon.primary}",
            }
        }
    }
}

#[component]
pub fn SecurityIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color: ColorTheme = use_context();

    rsx! {
        svg {
            "viewBox": "0 0 151 150",
            xmlns: "http://www.w3.org/2000/svg",
            height: "150",
            fill: "none",
            width: "151",
            path {
                "stroke-linecap": "round",
                fill: "{color.icon.primary}",
                "stroke-linejoin": "round",
                stroke: "{color.icon.primary}",
                "stroke-width": "1.5",
                d: "M75.4878 12.5C57.6753 12.5 46.1315 25.1188 32.4753 29.7188C26.9253 31.5938 24.144 32.525 23.0253 33.8438C21.9003 35.1562 21.569 37.0875 20.9128 40.9375C13.869 82.1625 29.2628 120.275 65.9691 135.113C69.9128 136.706 71.8878 137.5 75.5065 137.5C79.1253 137.5 81.1065 136.7 85.0503 135.106C121.757 120.275 137.132 82.1625 130.088 40.9375C129.425 37.0875 129.1 35.1562 127.975 33.8375C126.85 32.5188 124.075 31.5875 118.525 29.7188C104.863 25.1188 93.3003 12.5 75.4878 12.5Z",
            }
            path {
                stroke: "{color.icon.secondary}",
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M75.5 100C76.375 100 77.1875 99.6 78.8063 98.8125L91.8437 92.4187C97.6125 89.5875 100.5 88.175 100.5 85.9375V57.8125M75.5 100C74.625 100 73.8125 99.6 72.1937 98.8125L59.1563 92.4187C53.3875 89.5875 50.5 88.175 50.5 85.9375V57.8125M75.5 100V71.875M100.5 57.8125C100.5 55.575 97.6125 54.1625 91.8437 51.3313L78.8125 44.9437C77.1812 44.15 76.3687 43.75 75.5 43.75C74.625 43.75 73.8125 44.15 72.1937 44.9375L59.1563 51.3375C53.3875 54.1625 50.5 55.575 50.5 57.8125M100.5 57.8125C100.5 60.05 97.6125 61.4625 91.8437 64.2937L78.8063 70.6813C77.1813 71.475 76.3687 71.875 75.5 71.875M50.5 57.8125C50.5 60.05 53.3875 61.4625 59.1563 64.2937L72.1937 70.6813C73.8187 71.475 74.6313 71.875 75.5 71.875",
                "stroke-width": "1.5",
            }
        }
    }
}

#[component]
pub fn PublicIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color: ColorTheme = use_context();

    rsx! {
        svg {
            "viewBox": "0 0 151 150",
            xmlns: "http://www.w3.org/2000/svg",
            width: "151",
            fill: "none",
            height: "150",
            path {
                fill: "{color.icon.primary}",
                d: "M78.7231 138.867L78.4887 138.829C78.5215 138.854 78.5601 138.871 78.6011 138.878C78.642 138.885 78.6839 138.881 78.7231 138.867ZM132.014 55.0137C131.947 54.6602 131.764 54.3391 131.494 54.1015C131.224 53.8639 130.882 53.7234 130.523 53.7022C130.164 53.681 129.808 53.7803 129.512 53.9845C129.216 54.1887 128.996 54.4859 128.888 54.8291C128.785 55.1549 128.581 55.4396 128.305 55.6422C128.03 55.8449 127.698 55.9551 127.356 55.957H122.824C122.53 55.957 122.243 55.877 121.992 55.7256L115.435 51.7939C115.184 51.6425 114.896 51.5625 114.603 51.5625H104.115C103.795 51.5625 103.483 51.6573 103.218 51.835L90.2836 60.457C90.06 60.6059 89.8771 60.8082 89.7513 61.0455C89.6254 61.2828 89.5607 61.5478 89.5629 61.8164V74.0449C89.5626 74.338 89.642 74.6256 89.7927 74.877C89.9434 75.1283 90.1597 75.3339 90.4184 75.4717L107.378 84.5244C107.633 84.6615 107.847 84.8641 107.997 85.1113C108.148 85.3585 108.229 85.6413 108.234 85.9307L108.313 92.8125C108.317 93.0974 108.397 93.3761 108.544 93.6204C108.69 93.8647 108.899 94.066 109.148 94.2041L115.962 97.9746C116.215 98.1149 116.425 98.3199 116.572 98.5687C116.719 98.8174 116.797 99.1008 116.797 99.3897V113.628C116.797 113.939 116.887 114.244 117.056 114.505C117.225 114.766 117.465 114.974 117.749 115.102C118.033 115.23 118.347 115.273 118.655 115.227C118.963 115.181 119.25 115.047 119.484 114.841C122.232 112.421 126.172 108.888 126.697 108.079C127.564 106.737 128.377 105.363 129.137 103.957C130.711 101.043 132.043 98.0059 133.122 94.875C136.836 84.1201 133.901 64.7871 132.014 55.0137ZM84.4067 88.7109L66.4389 75.2344C66.2361 75.0822 65.9893 75 65.7358 75H57.2104C57.0645 75.0002 56.9201 74.9715 56.7854 74.9157C56.6506 74.8599 56.5282 74.778 56.4252 74.6748L52.4057 70.6553C52.2968 70.5465 52.1675 70.4602 52.0252 70.4014C51.883 70.3426 51.7305 70.3124 51.5766 70.3125H35.9936C35.774 70.3125 35.5593 70.2474 35.3767 70.1254C35.1941 70.0034 35.0518 69.83 34.9677 69.6271C34.8837 69.4242 34.8617 69.2009 34.9045 68.9855C34.9474 68.7701 35.0531 68.5723 35.2084 68.417L37.6752 65.9502C37.7782 65.847 37.9006 65.7651 38.0354 65.7093C38.1701 65.6535 38.3145 65.6248 38.4604 65.625H47.9438C48.4545 65.625 48.9513 65.4581 49.3585 65.1498C49.7658 64.8415 50.0611 64.4086 50.1996 63.917L52.2182 56.7393C52.2612 56.585 52.3354 56.4412 52.4362 56.3168C52.537 56.1923 52.6622 56.0898 52.8041 56.0156L60.852 51.873C61.0331 51.7795 61.1849 51.6378 61.2908 51.4637C61.3968 51.2895 61.4527 51.0896 61.4526 50.8857V47.2236C61.4525 46.9964 61.522 46.7745 61.6518 46.5879L65.9291 40.4297C66.0574 40.2447 66.2395 40.1035 66.4506 40.0254L72.436 37.7783C72.6477 37.699 72.8302 37.5569 72.959 37.371C73.0878 37.1852 73.1567 36.9644 73.1567 36.7383V33.3984C73.1567 33.2153 73.1115 33.035 73.0251 32.8735C72.9387 32.712 72.8139 32.5743 72.6616 32.4727L66.6616 28.4824C66.4967 28.374 66.3062 28.3109 66.1092 28.2996C65.9122 28.2883 65.7157 28.3291 65.5395 28.418L57.3715 32.502C57.1847 32.5942 56.9756 32.6319 56.7683 32.6106C56.561 32.5894 56.3639 32.5101 56.1996 32.3818L52.3266 29.3203C52.1944 29.214 52.0883 29.0789 52.0163 28.9254C51.9443 28.7718 51.9084 28.6038 51.9112 28.4342C51.914 28.2647 51.9555 28.098 52.0326 27.9469C52.1097 27.7958 52.2203 27.6643 52.3559 27.5625L55.5024 25.2451C55.6448 25.1406 55.7602 25.0036 55.8389 24.8454C55.9176 24.6873 55.9573 24.5126 55.9547 24.336C55.9521 24.1593 55.9073 23.9859 55.8241 23.8301C55.7408 23.6743 55.6214 23.5407 55.476 23.4404L50.5746 20.0215C50.405 19.902 50.2051 19.8326 49.9979 19.8213C49.7907 19.8099 49.5845 19.8569 49.4028 19.957C47.6303 20.9268 42.4301 23.7979 40.5844 25.0898C32.0448 31.0787 25.1723 39.145 20.6157 48.5273C20.0825 49.6289 19.4233 50.751 19.3588 51.9609C19.2944 53.1709 18.3451 55.8721 17.9496 56.9648C17.8963 57.1126 17.875 57.2699 17.8871 57.4265C17.8992 57.5831 17.9445 57.7353 18.02 57.873L28.476 77.0859C28.5701 77.2606 28.7098 77.4064 28.8803 77.5078L39.8813 84.1113C40.024 84.1963 40.1458 84.3124 40.2376 84.4508C40.3293 84.5893 40.3888 84.7466 40.4116 84.9111L42.6147 100.89C42.6362 101.043 42.6892 101.19 42.7704 101.321C42.8516 101.453 42.9592 101.566 43.0864 101.654L51.6703 107.555C51.919 107.726 52.0922 107.986 52.1537 108.281L56.7065 129.902C56.732 130.029 56.7806 130.15 56.85 130.26C57.2778 130.951 58.9828 133.482 61.0395 133.857C60.8491 133.91 60.6791 134.03 60.4887 134.086C60.9827 134.173 61.4717 134.286 61.9535 134.426C62.5395 134.584 63.1254 134.719 63.7114 134.845C64.6284 135.026 64.7192 135.167 65.1586 134.353C65.7446 133.263 66.4155 132.888 66.9164 132.753C67.1232 132.705 67.3123 132.6 67.4621 132.45C67.6118 132.3 67.7162 132.11 67.7631 131.903L70.7133 118.228C70.7753 117.941 70.9426 117.688 71.1821 117.519L84.3657 108.173C84.5189 108.064 84.6438 107.921 84.73 107.754C84.8161 107.587 84.861 107.403 84.8608 107.215V89.6484C84.8625 89.4677 84.8224 89.2891 84.7436 89.1265C84.6648 88.9638 84.5495 88.8216 84.4067 88.7109Z",
            }
            path {
                fill: "{color.icon.primary}",
                d: "M77.2578 14.0625C77.2578 14.0625 76.1885 14.124 75.9717 14.1299C74.3838 14.1768 72.7998 14.2861 71.2197 14.458C65.4597 15.0831 59.8187 16.5284 54.4678 18.75C55.1797 19.2422 53.9551 19.6934 53.9551 19.6934L55.8711 23.4375H66.125L73.1562 26.9531L79.3086 23.4375L77.2578 14.0625ZM104.278 35.1738L108.998 31.0723C109.146 30.944 109.259 30.7809 109.328 30.5976C109.396 30.4144 109.418 30.2169 109.39 30.0232C109.363 29.8295 109.287 29.6458 109.171 29.4887C109.054 29.3317 108.9 29.2063 108.723 29.124L103.212 26.5664C102.932 26.4358 102.612 26.421 102.321 26.5253C102.03 26.6295 101.792 26.8443 101.659 27.123L99.3887 31.8721C99.2651 32.1317 99.2413 32.4277 99.3218 32.7038C99.4023 32.9798 99.5814 33.2166 99.8252 33.3691L102.89 35.2852C103.103 35.4176 103.352 35.4783 103.602 35.4582C103.852 35.4382 104.089 35.3385 104.278 35.1738ZM126.371 41.2119L124.663 38.5752C124.637 38.5342 124.613 38.4932 124.59 38.4492C124.282 37.8193 121.736 32.6777 119.609 30.6826C118.013 29.1738 117.559 29.6074 117.433 29.9414C117.361 30.1261 117.24 30.288 117.084 30.4102L108.635 37.2393C108.427 37.4076 108.167 37.4996 107.899 37.5H103.525C103.371 37.4999 103.219 37.5301 103.077 37.5889C102.934 37.6477 102.805 37.734 102.696 37.8428L99.1807 41.3584C99.0717 41.4672 98.9853 41.5965 98.9263 41.7387C98.8673 41.881 98.837 42.0335 98.837 42.1875C98.837 42.3415 98.8673 42.494 98.9263 42.6363C98.9853 42.7785 99.0717 42.9078 99.1807 43.0166L102.696 46.5322C102.805 46.641 102.934 46.7273 103.077 46.7861C103.219 46.8449 103.371 46.8751 103.525 46.875H125.548C125.706 46.8751 125.863 46.8433 126.008 46.7813C126.154 46.7193 126.285 46.6285 126.395 46.5144C126.504 46.4002 126.589 46.2651 126.645 46.1171C126.701 45.9691 126.726 45.8114 126.72 45.6533L126.559 41.8008C126.55 41.5912 126.485 41.3879 126.371 41.2119Z",
            }
            path {
                d: "M75.5 21.0938C87.9715 21.0927 100.058 25.4161 109.699 33.3272C119.34 41.2383 125.94 52.2476 128.374 64.4793C130.808 76.7111 128.926 89.4083 123.047 100.408C117.169 111.407 107.659 120.028 96.137 124.801C84.6152 129.575 71.7946 130.206 59.8598 126.586C47.925 122.967 37.6145 115.321 30.685 104.952C23.7555 94.5823 20.6358 82.1312 21.8575 69.7196C23.0791 57.308 28.5666 45.704 37.3848 36.8848C42.3781 31.8629 48.3179 27.8815 54.8605 25.1709C61.403 22.4604 68.4182 21.0746 75.5 21.0938ZM75.5 9.375C39.2598 9.375 9.875 38.7598 9.875 75C9.875 111.24 39.2598 140.625 75.5 140.625C111.74 140.625 141.125 111.24 141.125 75C141.125 38.7598 111.74 9.375 75.5 9.375Z",
                fill: "{color.icon.primary}",
            }
        }
    }
}
