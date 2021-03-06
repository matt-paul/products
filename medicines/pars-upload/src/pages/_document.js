import Document, { Html, Head, Main, NextScript } from 'next/document';
import { Header } from '../header';
import { Footer } from '../footer';

class MyDocument extends Document {
  static async getInitialProps(ctx) {
    const initialProps = await Document.getInitialProps(ctx);
    return { ...initialProps };
  }

  render() {
    return (
      <Html
        lang="en"
        className="govuk-template js history flexbox no-flexboxtweener fixedsticky-withoutfixedfixed"
      >
        <Head>
          <title>Public Assessment Reports (PARs) upload</title>
        </Head>
        <body className="govuk-template__body">
          <a href="#main-content" class="govuk-skip-link">
            Skip to main content
          </a>

          <Header />

          <div className="govuk-width-container">
            <main className="govuk-main-wrapper " id="main-content" role="main">
              <Main />
            </main>
          </div>

          <Footer />

          <NextScript />
        </body>
      </Html>
    );
  }
}

export default MyDocument;
