import PageLayout from '@/components/PageLayout'

export default function Desktop() {
  return (
    <>
      <PageLayout pageTitle={"Desktop"} includeSearchBar={true}>

        <div>
          <h1>
            Help
          </h1>

          <div>
            some help content goes here
          </div>
        </div>
      </PageLayout>
    </>
  )
}
